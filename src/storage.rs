use std::path::Path;

const PATH: &str = "./storage.bin";

pub fn exists() -> bool {
    Path::new(PATH).exists()
}
