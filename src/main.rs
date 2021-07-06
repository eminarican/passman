mod storage;
mod app;

fn main() {
    let matches = app::get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let password = matches.value_of("secret").unwrap();
        println!("new {}", password)
    }

    if storage::exists() {
        if let Some(matches) = matches.subcommand_matches("set") {
            let password = matches.value_of("secret").unwrap();
            println!("set {}", password)
        } else if let Some(matches) = matches.subcommand_matches("get") {
            let password = matches.value_of("secret").unwrap();
            println!("get {}", password)
        } else if let Some(matches) = matches.subcommand_matches("gen") {
            let password = matches.value_of("secret").unwrap();
            println!("gen {}", password)
        }
    } else {
        println!("You should create new password file type --help for more information")
    }
}
