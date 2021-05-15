use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
    pub username: String,
    pub token: String
}

impl GitHub {
    pub fn new(user: &str, token: &str) -> GitHub {
        GitHub { username: user.to_string(), token: token.to_string() }
    }
}