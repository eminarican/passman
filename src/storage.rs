use std::collections::HashMap;
use clap::ArgMatches;
use std::path::Path;
use std::process::exit;
use rand::Rng;
use rand::distributions::Alphanumeric;
use cocoon::{Cocoon, Error};
use std::fs::File;
use std::result::Result::Err;

const PATH: &str = "./storage.bin";
const MAGIC: u32 = 0xDEADBEEF;

pub fn new(matches: &ArgMatches) -> Storage {
    let mut storage = Storage {
        magic: MAGIC,
        passwords: Some(HashMap::new()),
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
    passwords: Option<HashMap<String, String>>,
    secret: String
}

impl Storage {
    pub fn set(&mut self, provider: String, value: String) -> bool {
        if let None = self.get(provider.clone()) {
            self.passwords.as_mut().unwrap().insert(provider, value);
            true
        } else {
            false
        }
    }

    pub fn get(&mut self, provider: String) -> Option<String> {
        if let Some(password) = self.passwords.as_mut().unwrap().get(provider.as_str()) {
            Some(password.clone())
        } else {
            None
        }
    }

    pub fn gen(&mut self, provider: String) -> bool {
        let mut generated: String = generate();
        let mut eligible = false;

        while !eligible {
            let mut unique = true;

            for (_, password) in self.passwords.as_mut().unwrap().iter() {
                if *password == generated {
                    unique = false
                }
            }

            if unique {
                eligible = true
            } else {
                generated = generate()
            }
        }
        self.set(provider, "".to_string())
    }

    pub fn del(&mut self, provider: String) -> bool {
        if let None = self.get(provider.clone()) {
            let _ = self.passwords.as_mut().unwrap().remove(provider.as_str());
            true
        } else {
            false
        }
    }

    pub fn save(&self) {
        let encoded = bincode::serialize(self).unwrap();
        let cocoon = Cocoon::new(self.secret.as_bytes());
        let path = Path::new(PATH);
        let mut file: File;

        if path.exists() {
            file = File::open(path).unwrap();
        } else {
            file = File::create(path).unwrap();
        }
        let _ = cocoon.dump(encoded, &mut file);
    }

    fn load(&mut self) -> bool {
        let cocoon = Cocoon::new(self.secret.as_bytes());
        let buffer = std::fs::read(Path::new(PATH)).unwrap();
        if let Ok(buffer) = cocoon.unwrap(&buffer) {
            match bincode::deserialize::<Storage>(&buffer[..]) {
                Ok(storage) => {
                    self.passwords = storage.passwords;
                    true
                }
                Err(err) => {
                    eprintln!("{}", err);
                    false
                }
            }
        } else {
            false
        }
    }
}

fn generate() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
