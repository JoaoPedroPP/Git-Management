pub fn check_merge_methods(method: String) -> Result<(), String> {
    let allow_method = ["merge", "squash", "rebase"].to_vec();
    if allow_method.contains(&method.as_str()) {
        Ok(())
    }
    else {
        Err("Method not allowed. Choose one between merge, squash or rebase".to_string())
    }
}

pub fn check_states(method: String) -> Result<(), String> {
    let allow_method = ["open", "close", "all"].to_vec();
    if allow_method.contains(&method.as_str()) {
        Ok(())
    }
    else {
        Err("State not allowed. Choose one between open, close or all".to_string())
    }
}