use crate::services::GitHub;
use crate::services::RepoCreation;
use crate::services::RepoArchive;

use serde_json;

pub fn repo(git: clap::ArgMatches) {
    match git.value_of("action") {
        Some("create") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    let org = if git.is_present("org") {
                        git.value_of("org")
                    } else {
                        Some("")
                    };
                    let private = git.is_present("private");
                    let auto_init = git.is_present("auto_init");
                    creation(repo_name.to_string(), org.unwrap().to_string(), private, auto_init)
                },
                None => println!("Repo name required")
            }
        },
        Some("delete") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    let org = if git.is_present("org") {
                        git.value_of("org")
                    } else {
                        Some("")
                    };
                    delete_repo(repo_name.to_string(), org.unwrap().to_string())
                },
                None => println!("Repo name required")
            }
        },
        Some("archive") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    let org = if git.is_present("org") {
                        git.value_of("org")
                    } else {
                        Some("")
                    };
                    archive_repo(repo_name.to_string(), org.unwrap().to_string())
                },
                None => println!("Repo name required")
            }
        },
        Some("update") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    let org = if git.is_present("org") {
                        git.value_of("org")
                    } else {
                        Some("")
                    };
                    let description = if git.is_present("description") {
                        git.value_of("description")
                    } else {
                        Some("")
                    };
                    let private = git.is_present("private");
                    update(repo_name.to_string(), org.unwrap().to_string(), private, description.unwrap().to_string())
                },
                None => println!("Repo name required")
            }
        },
        Some(_) => println!("Invalid action"),
        None => println!("Type of action required")
    }
}

fn creation(name: String, org: String, private: bool, autoinit: bool) {
    println!("Creating repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/user/repos", base_url)
    } else {
        format!("{}/orgs/{org}/repos", base_url, org=org)
    };
    let payload = RepoCreation::new(name, private, autoinit);
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
                println!("Not possible to create repository");
                println!("Try update your credentials with");
                println!("gitmgt config -u <github username> -t <github token>");
            }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}

fn delete_repo(name: String, org: String) {
    println!("Deleting repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    // let url = format!("{}/repos/{}/{}", base_url, cred.username, name);
    let url = if org.is_empty() {
        format!("{}/repos/{}/{}", base_url, cred.username, name)
    } else {
        format!("{}/repos/{owner}/{repo}", base_url, owner=org, repo=name)
    };
    let client = reqwest::blocking::Client::new();
    let resp = client.delete(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .send();
        
        match resp {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::from_u16(204).unwrap() {
                    println!("Repository was deleted");
                }
                else {
                    println!("Not possible to delete repository");
                    println!("Try update your credentials with");
                    println!("gitmgt config -u <github username> -t <github token>");
                }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}

fn archive_repo(name: String, org: String) {
    println!("Archiving  repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/repos/{}/{}", base_url, cred.username, name)
    } else {
        format!("{}/repos/{owner}/{repo}", base_url, owner=org, repo=name)
    };
    let payload = RepoArchive::new(name, true);
    let client = reqwest::blocking::Client::new();
    let resp = client.patch(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .json(&payload)
        .send();
        
        match resp {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::from_u16(200).unwrap() {
                    println!("Repository was archived");
                }
                else {
                    println!("Not possible to archive repository");
                    println!("Try update your credentials with");
                    println!("gitmgt config -u <github username> -t <github token>");
                }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}

fn update(name: String, org: String, private: bool, description: String) {
    println!("Updating repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/repos/{}/{}", base_url, cred.username, name)
    } else {
        format!("{}/repos/{owner}/{repo}", base_url, owner=org, repo=name)
    };
    let payload = RepoCreation::update(name, private, description);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .json(&payload)
        .send();
        
        match resp {
            Ok(response) => {
            if response.status() == reqwest::StatusCode::from_u16(200).unwrap() {
                println!("Updated");
            }
            else {
                println!("Not possible to update repository");
                println!("Try update your credentials with");
                println!("gitmgt config -u <github username> -t <github token>");
            }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}