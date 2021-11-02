use clap::{Arg, App};
mod services;

fn main() {
    let mut matches = App::new("Code Repository Manipulation with Command Line Interace")
        .version("1.0")
        .author("Joao Pedro Poloni Ponce <poloniponce@protonmail.ch>")
        .arg(
            Arg::with_name("target")
                .help("Selected target: config_github | github")
                .takes_value(true)
                .index(1)
                .required(true)
        )
        .arg(
            Arg::with_name("action")
                .help("Select an action: create | delete | archive | update | pullrequest")
                .takes_value(true)
                .index(2)
        )
        .arg(
            Arg::with_name("name")
                .help("repo name")
                .short("n")
                .long("name")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("org")
                .help("org name")
                .short("o")
                .long("org")
                .default_value("")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("private")
                .help("is the repo private")
                .short("p")
                .long("private")
        )
        .arg(
            Arg::with_name("description")
                .help("description of the repo")
                .short("d")
                .long("description")
                .default_value("")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("auto_init")
                .help("the repo has auto_init")
                .short("i")
                .long("autoinit")
        )
        .arg(
            Arg::with_name("username")
                .help("git username")
                .short("u")
                .long("username")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("token")
                .help("git token")
                .short("t")
                .long("token")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("head")
                .help("head branch, the one you want to merge")
                .short("H")
                .long("head")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("base")
                .help("base branch, the one you want to insert changes from head branch")
                .short("B")
                .long("base")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("title")
                .help("title of the pull request")
                .short("T")
                .long("title")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("body")
                .help("body of the pull request")
                .short("b")
                .long("body")
                .default_value("")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("state")
                .help("state filter of the pull request")
                .short("s")
                .long("state")
                .validator(services::check_states)
                .default_value("all")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("merge_method")
                .help("merge method of the pull request")
                .short("M")
                .long("merge")
                .validator(services::check_merge_methods)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("pullrequest_number")
                .help("Number of the pull request")
                .short("N")
                .long("pullnumber")
                .takes_value(true)
        )
        .get_matches();

    if matches.args.get("target").unwrap().vals[0] == "create" ||
        matches.args.get("target").unwrap().vals[0] == "delete" ||
        matches.args.get("target").unwrap().vals[0] == "pullrequest" {
        let mut target = matches.args.get("target").unwrap().clone();
        matches.args.insert("action", target.clone());
        target.vals[0] = std::ffi::OsString::from("github".to_string());
        matches.args.insert("target", target.clone());
        for i in matches.args.values_mut() {
            if i.vals[0] != "github" {
                i.indices[0] = i.indices[0]+1;
            }
        }
    }

    match matches.value_of("target") {
        Some("config") => services::config_github(matches),
        Some("config_github") => services::config_github(matches),
        Some("github") => services::repo(matches),
        Some(_) => println!("Command not found"),
        None => println!("Invalid command"),
    };
}
