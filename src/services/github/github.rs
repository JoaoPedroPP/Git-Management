use crate::services::GitHub;
use crate::services::RepoCreation;
use crate::services::RepoArchive;
use crate::services::PullRequest;
use crate::services::MergePR;

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
                    let description = if git.is_present("description") {
                        git.value_of("description")
                    } else {
                        Some("")
                    };
                    let private = git.is_present("private");
                    let auto_init = git.is_present("auto_init");
                    creation(repo_name.to_string(), org.unwrap().to_string(), private, auto_init, description.unwrap().to_string())
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
        Some("pullrequest") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    match git.value_of("head") {
                        Some(head) => {
                            match git.value_of("base") {
                                Some(base) => {
                                    match git.value_of("title") {
                                        Some(title) => {
                                            let org = if git.is_present("org") {
                                                git.value_of("org")
                                            } else {
                                                Some("")
                                            };
                                            let body = if git.is_present("body") {
                                                git.value_of("body")
                                            } else {
                                                Some("")
                                            };
                                            pull_request(repo_name.to_string(), org.unwrap().to_string(), title.to_string(), head.to_string(), base.to_string(), body.unwrap().to_string())
                                        },
                                        None => println!("Title of the pull request necessary")
                                    }
                                },
                                None => println!("Base branch necessary to pull request")
                            }
                        },
                        None => println!("Head branch necessary to pull request")
                    }
                },
                None => println!("Repo name required")
            }
        },
        Some("listpr") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    let org = if git.is_present("org") {
                        git.value_of("org")
                    } else {
                        Some("")
                    };
                    let state = if git.is_present("state") {
                        git.value_of("state")
                    } else {
                        Some("all")
                    };
                    list_pr(repo_name.to_string(), org.unwrap().to_string(), state.unwrap().to_string());
                },
                None => println!("Repo name required")
            }
        },
        Some("merge") => {
            match git.value_of("name") {
                Some(repo_name) => {
                    match git.value_of("merge_method") {
                        Some(merge) => {
                            match git.value_of("pullrequest_number") {
                                Some(prn) => {
                                    let org = if git.is_present("org") {
                                        git.value_of("org")
                                    } else {
                                        Some("")
                                    };
                                    merge_pr(repo_name.to_string(), org.unwrap().to_string(), prn.to_string(), merge.to_string());

                                },
                                None => println!("Pull Request number required")
                            }
                        },
                        None => println!("Merge method required. merge|squash|rebase")
                    }
                },
                None => println!("Repo name required")
            }
        },
        Some(_) => println!("Invalid action"),
        None => println!("Type of action required")
    }
}

fn creation(name: String, org: String, private: bool, autoinit: bool, description: String) {
    println!("Creating repo...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/user/repos", base_url)
    } else {
        format!("{}/orgs/{org}/repos", base_url, org=org)
    };
    let payload = RepoCreation::new(name, private, autoinit, description);
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

// Pull request section
fn pull_request(name: String, org: String, title: String, head: String, base: String, body: String) {
    println!("Creating pull request...");
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/repos/{owner}/{repo}/pulls", base_url, owner=cred.username, repo=name)
    } else {
        format!("{}/repos/{owner}/{repo}/pulls", base_url, owner=org, repo=name)
    };
    let payload = PullRequest::new(title, head, base, body);
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
                    println!("Pull Request created at: {}", raw.get("html_url").unwrap());
                }
                else if response.status() == reqwest::StatusCode::from_u16(422).unwrap() {
                    let raw = response.json::<serde_json::Value>().unwrap();
                    println!("{}", raw.get("errors").unwrap()[0].get("message").unwrap());
                }
                else {
                    println!("Not possible to create the pull request");
                    println!("Try update your credentials with");
                    println!("gitmgt config -u <github username> -t <github token>");
                }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}

fn list_pr(name: String, org: String, state: String) {
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/repos/{owner}/{repo}/pulls?state={state}", base_url, owner=cred.username, repo=name, state=state)
    } else {
        format!("{}/repos/{owner}/{repo}/pulls?state={state}", base_url, owner=org, repo=name, state=state)
    };
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .send();

    match resp {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::from_u16(200).unwrap() {
                let raw = response.json::<serde_json::Value>().unwrap();
                for pr in raw.as_array().unwrap() {
                    println!("PR: {title}, State: {state}, Number: {number} -> {url}",
                        title=pr.get("title").unwrap().as_str().unwrap(),
                        state=pr.get("state").unwrap().as_str().unwrap(),
                        number=pr.get("number").unwrap().as_i64().unwrap(),
                        url=pr.get("html_url").unwrap().as_str().unwrap());
                }
            }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}

fn merge_pr(name: String, org: String, prn: String, merge: String) {
    let cred = GitHub::get_credentials();
    let base_url = "https://api.github.com";

    let url = if org.is_empty() {
        format!("{}/repos/{owner}/{repo}/pulls/{pull_number}/merge", base_url, owner=cred.username, repo=name, pull_number=prn)
    } else {
        format!("{}/repos/{owner}/{repo}/pulls/{pull_number}/merge", base_url, owner=org, repo=name, pull_number=prn)
    };
    let payload: MergePR = MergePR::new(merge);
    let client = reqwest::blocking::Client::new();
    let resp = client.put(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "reqwest")
        .basic_auth(&cred.username, Some(cred.token))
        .json(&payload)
        .send();

    match resp {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::from_u16(200).unwrap() {
                let raw = response.json::<serde_json::Value>().unwrap();
                println!("{}", raw.get("message").unwrap());
            }
            else if response.status() == reqwest::StatusCode::from_u16(403).unwrap() {
                println!("Forbidden operation, review your permissions on the repo");
            }
            else if response.status() == reqwest::StatusCode::from_u16(404).unwrap() {
                println!("Repo not found, may check repo name");
            }
            else if response.status() == reqwest::StatusCode::from_u16(405).unwrap() {
                let raw = response.json::<serde_json::Value>().unwrap();
                println!("{}", raw.get("message").unwrap());
            }
            else if response.status() == reqwest::StatusCode::from_u16(409).unwrap() {
                let raw = response.json::<serde_json::Value>().unwrap();
                println!("{}", raw.get("message").unwrap());
            }
            else if response.status() == reqwest::StatusCode::from_u16(422).unwrap() {
                println!("Unable to process");
            }
            else {
                println!("Not possible to process your request now, try again later.");
            }
        },
        Err(error) => {
            println!("API request not successfull: {}", error);
        }
    }
}