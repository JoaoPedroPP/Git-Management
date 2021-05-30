use serde::{Deserialize, Serialize};
use serde_json;
use dirs::home_dir;
use std::path::Path;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
    pub username: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoCreation {
    pub name: String,
    pub private: bool,
    pub description: String,
    pub auto_init: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoUpdate {
    pub name: String,
    pub private: bool,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoArchive {
    pub name: String,
    pub archived: bool
}

impl GitHub {
    pub fn new(user: &str, token: &str) -> GitHub {
        GitHub { username: user.to_string(), token: token.to_string() }
    }

    pub fn get_credentials() -> GitHub {
        let home: String = home_dir().unwrap().to_str().unwrap().to_string();
        let path = format!("{}/.gitmgt/github.json", home);
        let folder = Path::new(&path);
        let content = fs::read_to_string(folder).unwrap();
        let cred: GitHub = serde_json::from_str(&content).unwrap();

        cred
    }
}

impl RepoCreation {
    pub fn new(name: String, private: bool, autoinit: bool) -> RepoCreation {
        RepoCreation { name: name, private: private, description: String::from(""), auto_init: autoinit }
    }

    
    pub fn update(name: String, private: bool, description: String) -> RepoUpdate {
        RepoUpdate { name: name, private: private, description }
    }
}

impl RepoArchive {
    pub fn new(name: String, arch: bool) -> RepoArchive {
        RepoArchive { name: name, archived: arch }
    }
}