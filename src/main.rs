use std::{collections::HashMap, process::{self}, path::{Path, PathBuf}, fs};

use anyhow::Result;
use serde_json::{json, Value};

fn main() -> Result<()> {

    println!("{}", dirs::config_dir().unwrap().join("data.json").to_str().unwrap());
    let client = Client::new()?;
    let post = client.make_request(format!("pcs/get-actions/{}", client.pc_id), None)?;
    println!("{}", post);
    let action_map: Vec<HashMap<String, String>> = serde_json::from_str(post.as_str())?;
    println!("{:?}", action_map);

    Ok(())
}

struct Client {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub pc_id: String,
    token: String,
    
    data_file: PathBuf,
}

impl Client {
    pub fn new() -> Result<Self> {
        let mut client = Client {
            client: reqwest::blocking::Client::new(),
            base_url: "http://localhost:3000".to_owned(), //to figure out later
            pc_id: "0001".to_owned(), //to figure out later
            token: "12345".to_owned(), //to figure out later 
            data_file: dirs::config_dir().unwrap().join("data.json"),
        };
        client.load_data()?;
        Ok(client)
    }

    pub fn make_request(&self, path: String, data: Option<Value>) -> reqwest::Result<String> {
        let mut body = json!({"token": self.token});
        if let Some(obj) = data {
            for (k, v) in obj.as_object().expect("Invalid JSON object!") {
                body.as_object_mut().unwrap().insert(k.clone(), v.clone());
            }
        }   
        let post = self.client
            .post(format!("{}/{}", self.base_url, path))
            .body(body.to_string())
            .send();

        post?.text()
    }

    fn save_data(&self) -> Result<()> {
        if !Path::exists(&self.data_file) {
            fs::File::create(&self.data_file).expect("Could not create data file!");
        }
        fs::write(&self.data_file, json!(
            {
                "pc_id": &self.pc_id,
                // "token": &self.token,
                "base_url": &self.base_url,
            }
        ).to_string())?;

        Ok(())
    }
    fn load_data(&mut self) -> Result<()> {
        if !Path::exists(&self.data_file) {
            self.save_data()?;
        } else {
            let data = fs::read_to_string(&self.data_file)?;
            let mut json: HashMap<String, String> = serde_json::from_str(&data)?;
            self.pc_id = json.remove("pc_id").unwrap();
            self.base_url = json.remove("base_url").unwrap();
        }
        Ok(())
    }
}

fn exec_powershell_cmd(cmd: String) {
    if cfg!(target_os = "windows") {
        process::Command::new("powershell").args(["-Command", &cmd]).output().expect("Failed to execute command!");
    } else {
        println!("Client not running on Windows! Unable to execute powershell command.")
    }
}
