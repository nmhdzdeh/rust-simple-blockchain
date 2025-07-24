use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = get_timestamp();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
    let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let genesis_block = Block::new(0, "First Block".to_string(), "0".to_string());
    println!("{:#?}", genesis_block);
}
