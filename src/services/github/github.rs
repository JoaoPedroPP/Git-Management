use crate::services::GitHub;
use crate::services::RepoCreation;

use serde_json;

pub fn create_repo(git: clap::ArgMatches) {
    match git.value_of("action") {
        Some("create") => {
            match git.value_of("name") {
                Some(repo_name) => creation(repo_name.to_string()),
                None => println!("Repo name required")
            }
        },
        Some(_) => println!("Invalid action"),
        None => println!("Type of action required")
    }
}

fn creation(name: String) {
    println!("Creating repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = format!("{}/user/repos", base_url);
    let payload = RepoCreation::new(name);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .json(&payload)
        .send();
        
        match resp {
            Ok(response) => {
            if response.status() == reqwest::StatusCode::from_u16(201).unwrap() {
                let raw = response.json::<serde_json::Value>().unwrap();
                println!("1. git remote add origin {}", raw.get("clone_url").unwrap());
                println!("2. git branch -M main");
                println!("3. git push -u origin main");
            }
            else {
                println!("Not possivle to create repository")
            }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}