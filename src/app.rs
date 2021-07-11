use clap::{App, Arg, ArgMatches};
use std::process::exit;

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
        .subcommand(App::new("list")
            .about("Lists providers")
        )
        .get_matches()
}

pub enum Subcommand {
    Set {
        provider: String,
        value: String
    },
    Get {
        provider: String
    },
    Gen {
        provider: String
    },
    Del {
        provider: String
    },
    List
}

pub fn subcommand(matches: &ArgMatches) -> Subcommand {
    return if let Some(matches) = matches.subcommand_matches("set") {
        Subcommand::Set {
            provider: matches.value_of("provider").unwrap().to_string(),
            value: matches.value_of("value").unwrap().to_string(),
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        Subcommand::Get {
            provider: matches.value_of("provider").unwrap().to_string(),
        }
    } else if let Some(matches) = matches.subcommand_matches("gen") {
        Subcommand::Gen {
            provider: matches.value_of("provider").unwrap().to_string(),
        }
    } else if let Some(matches) = matches.subcommand_matches("del") {
        Subcommand::Del {
            provider: matches.value_of("provider").unwrap().to_string(),
        }
    } else if let Some(_) = matches.subcommand_matches("list") {
        Subcommand::List
    } else {
        println!("Please use --help flag for gathering more info");
        exit(0)
    }
}
