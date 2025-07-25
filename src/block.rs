use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Block::get_timestamp(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn mine_block(index: u64, data: String, previous_hash: String, difficulty: usize) -> Block {
        let timestamp = Block::get_timestamp();
        let mut nonce = 0;
        let mut hash;

        loop {
            hash = Block::calculate_hash(index, timestamp, &data, &previous_hash, nonce);
            if hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }
            nonce += 1;
        }

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn get_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: u128,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
