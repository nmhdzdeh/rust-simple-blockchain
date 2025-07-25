use crate::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.blocks.last().expect("Blockchain is empty");
        let new_block = Block::mine_block(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
            self.difficulty,
        );
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.data,
                &current_block.previous_hash,
                current_block.nonce,
            );

            if current_block.hash != recalculated_hash {
                return false;
            }

            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
