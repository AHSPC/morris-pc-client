use std::collections::HashMap;
use reqwest;
use std::process::Command;
use std::error::Error;
use std::string::String;
use std::collections::HashMap;
use std::error::Error;
use std::collections::HashMap;
use tokio::time::{Duration, sleep};


const BASE_URL: &str = "http://localhost:3000/pcs";
const COMPUTER_ID: &str = "0001";
let mut TOKEN: &str = "12345";

type Data = HashMap<String, serde_json::Value>;

async fn make_request(path: &str, mut data: Data) -> Result<Data, reqwest::Error> {
    data.insert("token".to_string(), serde_json::Value::String(TOKEN.to_string()));

    let client = reqwest::Client::new();
    let url = format!("{}/{}/{}", BASE_URL, path, COMPUTER_ID);
    let json_data = serde_json::to_string(&data)?;

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {}", resp.status())));
    }

    let body = resp.text().await?;
    let response: Data = serde_json::from_str(&body).unwrap_or_else(|_| {
        eprintln!("{} (body is not Data) 3 {}", path, &body);
        Data::from([("text", serde_json::Value::String(body))])
    });

    println!("5 {:?}", &response);
    Ok(response)
}


fn exec_cmd(cmd: &str) -> Result<String, Box<dyn Error>> {
    let mut shell_cmd = Command::new("powershell")
        .arg("-Command")
        .arg(cmd)
        .output()?;

    if !shell_cmd.status.success() {
        return Err(format!("Command execution failed: {}", shell_cmd.status).into());
    }

    let stdout = String::from_utf8(shell_cmd.stdout)?;
    Ok(stdout)
}

async fn check_tasks() {
    match make_request("/get-actions", HashMap::new()).await {
        Ok(resp) => {
            for (id, action) in resp {
                match exec_cmd(&action.as_str().unwrap_or_default()).await {
                    Ok(_) => make_request("/mark-completed", [("task_id", id)]),
                    Err(err) => make_request("/mark-failed", [("task_id", id), ("info", err.to_string())]),
                }
            }
        }
        Err(err) => eprintln!("1 - Error getting actions: {}", err),
    }
}


#[tokio::main]
async fn main() {
    if let Err(err) = make_request("/exists", HashMap::new()).await {
        eprintln!("Error, /exists request failed: {}", err);
    }

    if let Ok(config) = make_request("/get-config", HashMap::new()).await {
        println!("{:?}", config);
    }

    check_tasks().await;

    let tick_duration = Duration::from_secs(20);
    loop {
        sleep(tick_duration).await;
        check_tasks().await;
    }
}
