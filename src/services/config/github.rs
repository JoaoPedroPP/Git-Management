use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct GitHub {
    username: String,
    token: String
}

impl GitHub {
    fn new(user: &str, token: &str) -> GitHub {
        GitHub { username: user.to_string(), token: token.to_string() }
    }
}

pub fn github(git: clap::ArgMatches) {
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

fn set_github(user: &str, token: &str) {
    let cred: GitHub = GitHub::new(user, token);
    let text = serde_json::to_vec(&cred).unwrap();
    let mut file = File::create("cred.json").unwrap();
    file.write_all(&text).unwrap();
    println!("Credentials upadated");
}