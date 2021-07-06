use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use clap::ArgMatches;
use std::path::Path;

const PATH: &str = "./storage.bin";
const MAGIC: u32 = 0xDEADBEEF;

pub fn new(matches: &ArgMatches) -> Storage {
    let storage = Storage {
        magic: MAGIC,
        passwords: HashMap::new(),
        secret: String::from(matches.value_of("secret").unwrap())
    };

    if !Path::new(PATH).exists() {
        println!("created new password file");
        storage.save()
    } else {
        storage.load()
    }

    storage
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    magic: u32,
    passwords: HashMap<String, String>,
    #[serde(skip_serializing)]
    secret: String
}

impl Storage {
    pub fn save(&self) {}
    fn load(&self) {}
}
