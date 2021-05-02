use clap::{Arg, App};
mod services;

fn main() {
    let matches = App::new("Code Repository Manipulation with Command Line Interace")
        .version("1.0")
        .author("Joao Pedro Poloni Ponce <poloniponce@protonmail.ch>")
        .arg(
            Arg::with_name("target")
                .help("Selected target")
                .takes_value(true)
                .index(1)
                .required(true)
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
        .get_matches();

    match matches.value_of("target") {
        Some("config") => services::github(matches),
        Some(_) => println!("Command not found"),
        None => println!("Invalid command"),
    };
}
