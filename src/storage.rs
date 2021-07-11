use std::collections::HashMap;
use clap::ArgMatches;
use std::path::Path;
use std::process::exit;
use rand::Rng;
use rand::distributions::Alphanumeric;
use cocoon::Cocoon;
use std::fs::File;

const PATH: &str = "./storage.bin";

pub fn new(matches: &ArgMatches) -> Storage {
    let mut storage = Storage {
        passwords: HashMap::new(),
        secret: String::from(matches.value_of("secret").unwrap())
    };

    if !Path::new(PATH).exists() {
        println!("created new password file");
    } else {
        if !storage.load() {
            println!("either secret isn't correct or password file corrupted");
            exit(0)
        }
    }

    storage
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    passwords: HashMap<String, String>,
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

    pub fn get(&mut self, provider: String) -> Option<String> {
        if let Some(password) = self.passwords.get(provider.as_str()) {
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

            for (_, password) in self.passwords.iter() {
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
        self.set(provider, generated)
    }

    pub fn del(&mut self, provider: String) -> bool {
        if let Some(_) = self.get(provider.clone()) {
            let _ = self.passwords.remove(provider.as_str());
            true
        } else {
            false
        }
    }

    pub fn save(&self) {
        let cocoon = Cocoon::new(self.secret.as_bytes());
        let mut file = File::create(Path::new(PATH)).unwrap();
        let serialized = bincode::serialize(self).unwrap();
        let _ = cocoon.dump(serialized, &mut file);
    }

    fn load(&mut self) -> bool {
        let cocoon = Cocoon::new(self.secret.as_bytes());
        let mut file = File::open(PATH).unwrap();

        if let Ok(data) = cocoon.parse(&mut file) {
            if let Ok(deserialized) = bincode::deserialize::<Storage>(&data[..]) {
                self.passwords = deserialized.passwords.clone();
                true
            } else {
                false
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
