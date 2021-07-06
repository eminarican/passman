mod storage;

use clap::{App, Arg};

fn main() {
    let secret = Arg::new("secret")
        .about("Sets secret")
        .value_name("password")
        .takes_value(true)
        .required(true)
        .long("secret")
        .short('s');
    let provider = Arg::new("provider")
        .about("Sets provider")
        .value_name("provider")
        .required(true);
    let matches = App::new("Passman")
        .version("0.1.0")
        .about("Personal password manager")
        .subcommand(App::new("new")
            .about("Creates new password file")
            .arg(Arg::new("secret")
                .about("Sets secret")
                .value_name("password")
                .required(true)
            )
        )
        .subcommand(App::new("set")
            .about("Sets a new password")
            .arg(&secret)
            .arg(&provider)
            .arg(Arg::new("value")
                .about("Sets value")
                .value_name("value")
                .required(true)
            )
        )
        .subcommand(App::new("get")
            .about("Gets a password")
            .arg(&secret)
            .arg(&provider)
        )
        .subcommand(App::new("gen")
            .about("Generates a password")
            .arg(&secret)
            .arg(&provider)
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        let password = matches.value_of("secret").unwrap();
        println!("new {}", password)
    } else if let Some(matches) = matches.subcommand_matches("set") {
        let password = matches.value_of("secret").unwrap();
        println!("set {}", password)
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let password = matches.value_of("secret").unwrap();
        println!("get {}", password)
    } else if let Some(matches) = matches.subcommand_matches("gen") {
        let password = matches.value_of("secret").unwrap();
        println!("gen {}", password)
    }
}
