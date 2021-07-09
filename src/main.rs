#[macro_use]
extern crate serde_derive;
extern crate bincode;

use app::Subcommand;

mod storage;
mod app;

fn main() {
    let matches = app::new();
    let mut storage = storage::new(&matches);

    match app::subcommand(&matches) {
        Subcommand::Set{ provider, value } => {
            if storage.set(provider, value) {
                println!("new password has been successfully set for the provider")
            } else {
                println!("couldn't set a new password for provider, delete old one")
            }
        }
        Subcommand::Get{ provider } => {
            if let Some(password) = storage.get(provider.clone()) {
                println!("{}: {}", provider, password)
            } else {
                println!("there's no password set for specified provider")
            }
        }
        Subcommand::Gen{ provider } => {
            if storage.gen(provider) {
                println!("successfully generated new password for specified provider")
            } else {
                println!("couldn't generate a new password for provider, delete old one")
            }
        }
        Subcommand::Del{ provider } => {
            if storage.del(provider) {
                println!("successfully deleted password for specified provider")
            } else {
                println!("couldn't delete password for provider, create new one")
            }
        }
    }
    storage.save()
}
