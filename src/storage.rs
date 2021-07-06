use std::collections::HashMap;
use clap::ArgMatches;
use std::path::Path;
use std::process::exit;

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
        if !storage.load() {
            println!("secret isn't correct");
            exit(0)
        }
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
    pub fn set(&mut self, provider: String, value: String) -> bool {
        if let None = self.get(provider.clone()) {
            self.passwords.insert(provider, value);
            true
        } else {
            false
        }
    }

    pub fn get(&self, provider: String) -> Option<String> {
        if let Some(password) = self.passwords.get(provider.as_str()) {
            Some(password.clone())
        } else {
            None
        }
    }

    pub fn gen(&mut self, provider: String) -> bool {
        // Todo: generate password
        self.set(provider, "".to_string())
    }

    pub fn del(&mut self, provider: String) -> bool {
        if let None = self.get(provider.clone()) {
            let _ = self.passwords.remove(provider.as_str());
            true
        } else {
            false
        }
    }

    pub fn save(&self) {}
    fn load(&self) -> bool {
        false
    }
}
