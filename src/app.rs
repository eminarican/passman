use clap::{App, Arg, ArgMatches};

pub fn new() -> ArgMatches {
    let provider = Arg::new("provider")
        .about("Sets provider")
        .value_name("provider")
        .required(true);
    App::new("Passman")
        .version("0.1.0")
        .about("Personal password manager")
        .arg(Arg::new("secret")
            .about("Sets secret")
            .value_name("password")
            .takes_value(true)
            .required(true)
            .long("secret")
            .short('s')
        )
        .subcommand(App::new("set")
            .about("Sets a new password")
            .arg(&provider)
            .arg(Arg::new("value")
                .about("Sets value")
                .value_name("value")
                .required(true)
            )
        )
        .subcommand(App::new("get")
            .about("Gets a password")
            .arg(&provider)
        )
        .subcommand(App::new("gen")
            .about("Generates a password")
            .arg(&provider)
        )
        .subcommand(App::new("del")
            .about("Deletes a password")
            .arg(&provider)
        )
        .get_matches()
}

pub enum Subcommand {
    Set,
    Get,
    Gen,
    Del,
}

pub fn subcommand(matches: &ArgMatches) -> Subcommand {
    return if matches.subcommand_matches("set") {
        Subcommand::Set
    } else if matches.subcommand_matches("get") {
        Subcommand::Get
    } else if matches.subcommand_matches("gen") {
        Subcommand::Gen
    } else if matches.subcommand_matches("del") {
        Subcommand::Del
    }
}
