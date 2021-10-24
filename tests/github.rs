use assert_cmd::Command;

#[test]
fn canary() {
    assert_eq!(true, true);
}

#[test]
fn create_simple_repo() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("github")
        .arg("create")
        .arg("-n")
        .arg("simple_repo")
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("3. git push -u origin main"), "CLI failed to create a simple repo");
}

#[test]
fn delete_simple_repo() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("github")
        .arg("delete")
        .arg("-n")
        .arg("simple_repo")
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Repository was deleted"), "CLI failed to delete simple repo")
}

#[test]
fn create_simple_private_repo() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("github")
        .arg("create")
        .arg("-n")
        .arg("simple_private_repo")
        .arg("-p")
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("3. git push -u origin main"), "CLI failed to create a simple repo");
}

#[test]
fn delete_simple_private_repo() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("github")
        .arg("delete")
        .arg("-n")
        .arg("simple_private_repo")
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Repository was deleted"), "CLI failed to delete simple repo")
}