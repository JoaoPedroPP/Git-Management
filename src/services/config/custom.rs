use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;
use dirs::home_dir;
use serde_json;

use crate::services::Custom;

pub fn config_custom(git: clap::ArgMatches) {
    match git.value_of("username") {
        Some(user) => {
            match git.value_of("token") {
                Some(token) => set_github(user, token),
                None => println!("Token required to create things")
            }
        },
        None => println!("Username required to create things")
    }
}

fn set_custom(url: &str, user: &str, token: &str) {
    let cred: Custom = Custom::new(url, user, token);
    let text = serde_json::to_vec(&cred).unwrap();
    let home: String = home_dir().unwrap().to_str().unwrap().to_string();
    let path = format!("{}/.gitmgt", home);
    let folder = Path::new(&path);
    match create_dir_all(folder) {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    };
    let file_path = format!("{}/.gitmgt/custom.json", home);
    let mut file = File::create(file_path).unwrap();
    file.write_all(&text).unwrap();
    println!("Credentials upadated");
}