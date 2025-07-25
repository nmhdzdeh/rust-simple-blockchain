use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use crate::blockchain::Blockchain;
use serde_json;

const FILE_PATH: &str = "blockchain.json";

pub fn save_blockchain(blockchain: &Blockchain) {
    let json = serde_json::to_string_pretty(blockchain).expect("Failed to serialize blockchain");
    let mut file = File::create(FILE_PATH).expect("Failed to create blockchain.json");
    file.write_all(json.as_bytes())
        .expect("Failed to write blockchain to file");
}

pub fn load_blockchain() -> Blockchain {
    if Path::new(FILE_PATH).exists() {
        let file = File::open(FILE_PATH).expect("Failed to open blockchain.json");
        let reader = BufReader::new(file);
        let blockchain:Blockchain= serde_json::from_reader(reader).unwrap_or_else(|_| Blockchain::new());

        if blockchain.is_valid(){
            blockchain
        }else{
            panic!("Blockchain data is invalid! Halting.");
        }
    } else {
        Blockchain::new()
    }
}
