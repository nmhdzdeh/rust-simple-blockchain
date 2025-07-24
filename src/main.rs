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

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let last_block = self.blocks.last().expect("Blockchain is empty");
        let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone());
        self.blocks.push(new_block);
    }
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
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First trasaction".to_string());
    blockchain.add_block("Second trasaction".to_string());

    for block in blockchain.blocks.iter() {
        println!("{:#?}", block);
    }
}
