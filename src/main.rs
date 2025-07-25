mod block;
mod blockchain;
mod storage;

use crate::blockchain::Blockchain;
use storage::{load_blockchain, save_blockchain};

fn main() {
    let mut blockchain = load_blockchain().unwrap_or_else(Blockchain::new);

    blockchain.add_block("First trasaction".to_string());
    blockchain.add_block("Second trasaction".to_string());

    for block in blockchain.blocks.iter() {
        println!("{:#?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());

    save_blockchain(&blockchain);
}
