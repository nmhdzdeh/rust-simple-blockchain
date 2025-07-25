use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::blockchain::Blockchain;
use serde_json;

const FILE_PATH: &str = "blockchain.json";

pub fn save_blockchain(blockchain: &Blockchain) {
    if let Ok(json) = serde_json::to_string_pretty(blockchain) {
        let _ = std::fs::write(FILE_PATH, json);
    }
}

pub fn load_blockchain() -> Option<Blockchain> {
    if Path::new(FILE_PATH).exists() {
        let mut file = File::open(FILE_PATH).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;
        serde_json::from_str(&content).ok()
    } else {
        None
    }
}
