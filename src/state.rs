use crate::storage::Storage;

pub struct State {
    pub show: bool,
    pub attempt: i8,
    pub provider: String,
    pub password: String,
    pub storage: Storage,
}

impl State {
    pub fn clear_fields(&mut self) {
        self.provider.clear();
        self.password.clear();
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            show: false,
            attempt: 3,
            provider: String::new(),
            password: String::new(),
            storage: Storage::default(),
        }
    }
}

pub enum Message {
    None,
    Login {
        password: String,
    },
    Remove {
        provider: String,
    },
    Copy {
        provider: String,
    },
    Generate {
        provider: String,
    },
    Add {
        provider: String,
        password: String,
    },
}
