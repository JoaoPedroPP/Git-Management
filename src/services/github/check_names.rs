pub fn check_merge_methods(method: String) -> Result<(), String> {
    let allow_method = ["merge", "squash", "rebase"].to_vec();
    if allow_method.contains(&method.as_str()) {
        Ok(())
    }
    else {
        Err("Method not allowed. Choose one between merge, squash or rebase".to_string())
    }
}