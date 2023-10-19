package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os/exec"
	"time"
)

const (
	baseURL    = "http://localhost:3000/pcs"
	computerID = "0001"
	token      = "12345"
)

type Data = map[string]interface{}

func makeRequest(path string, data Data) (Data, error) {
	data["token"] = token

	jsonData, err := json.Marshal(data)
	if err != nil {
		return nil, err
	}

	resp, err := http.Post(baseURL+path+"/"+computerID, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP error: %s", resp.Status)
	}

	var response Data
	body, _ := io.ReadAll(resp.Body)
	err = json.Unmarshal(body, &response)
	if err != nil {
		fmt.Println(path, "(body is not Data) 3", string(body))
		return Data{"text": string(body)}, nil
	}

	fmt.Println(5, response)
	return response, nil
}

func execCmd(cmd string) (string, error) {
	shellCmd := exec.Command("powershell", "-Command", cmd)

	output, err := shellCmd.CombinedOutput()
	if err != nil {
		return "", err
	}

	return string(output), nil
}

func checkTasks() {
	resp, err := makeRequest("/get-actions", Data{})
	if err != nil {
		fmt.Println("1 - Error getting actions.")
		return
	}

	for id, action := range resp {
		_, err := execCmd(action.(string))
		if err != nil {
			makeRequest("/mark-failed", Data{"task_id": id, "info": err.Error()})
			continue
		}

		makeRequest("/mark-completed", Data{"task_id": id})
	}
}

func main() {
	_, err := makeRequest("/exists", Data{})
	if err != nil {
		fmt.Println("Error, /exists request failed.")
	}

	config, _ := makeRequest("/get-config", Data{})
	fmt.Println(config)

	checkTasks()

	for range time.Tick(20 * time.Second) {
		checkTasks()
	}
}
