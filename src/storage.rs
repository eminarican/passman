use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Cursor, Read}, path::Path};
use rand::{distributions::Alphanumeric, Rng};

use bincode::{config, Decode, Encode};
use cocoon::Cocoon;

#[derive(Encode, Decode, Clone, Default)]
pub struct Storage {
    passwords: PasswordMap,
    secret: String,
}

impl Storage {
    pub fn new(secret: String) -> Option<Storage> {
        if !storage_path().exists() {
            let storage = Storage {
                passwords: Default::default(),
                secret,
            };

            storage.save();
            return Some(storage)
        }

        Self::load(secret)
    }

    pub fn len(&mut self) -> usize {
        self.passwords.len()
    }

    pub fn _get(&self, provider: String) -> Option<&String> {
        self.passwords.get(provider.as_str())
    }

    pub fn set(&mut self, provider: String, password: String) -> bool {
        if self.passwords.contains_key(provider.as_str()) {
            return false
        }

        self.passwords.insert(provider, password).is_none()
    }

    pub fn del(&mut self, provider: String) -> bool {
        self.passwords.remove(provider.as_str()).is_some()
    }

    pub fn gen(&mut self, provider: String) -> bool {
        let generated = loop {
            let password = generate();
            if !self.passwords.values().any(|v| v == &password) {
                break password;
            }
        };

        self.set(provider, generated)
    }

    pub fn load(secret: String) -> Option<Storage> {
        let mut file = storage_file().ok()?;

        let mut data = Vec::new();
        file.read_to_end(&mut data).ok()?;

        let cocoon = Cocoon::new(secret.as_bytes());
        let decrypted = cocoon.parse(&mut Cursor::new(data)).ok()?;

        let (storage, _) = bincode::decode_from_slice(
            &decrypted,
            config::standard()
        ).ok()?;
        Some(storage)
    }

    pub fn save(&self) -> bool {
        let cocoon = Cocoon::new(self.secret.as_bytes());

        let file = storage_file();
        if file.is_err() {
            return false
        }

        let serialized = bincode::encode_to_vec(
            self, config::standard()
        );
        if serialized.is_err() {
            return false
        }

        cocoon.dump(serialized.unwrap(), &mut file.unwrap()).is_ok()
    }
}

type PasswordMap = HashMap<String, String>;

impl IntoIterator for Storage {
    type Item = <PasswordMap as IntoIterator>::Item;
    type IntoIter = <PasswordMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.passwords.into_iter()
    }
}

const PATH: &str = "./passwords.db";

fn storage_path<'a>() -> &'a Path {
    Path::new(PATH)
}

fn storage_file() -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATH)
}

fn generate() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
