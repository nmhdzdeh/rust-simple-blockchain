mod block;
mod blockchain;

use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First trasaction".to_string());
    blockchain.add_block("Second trasaction".to_string());

    for block in blockchain.blocks.iter() {
        println!("{:#?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
