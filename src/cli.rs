use crate::storage::{load_blockchain, save_blockchain};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RustChain")]
#[command(about="Simple Blockchain CLI", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { data: String },
    Show,
    Validate,
}

pub fn run() {
    let cli = Cli::parse();
    let mut blockchain = load_blockchain();

    match &cli.command {
        Commands::Add { data } => {
            blockchain.add_block(data.to_string());
            save_blockchain(&blockchain);
            println!("Block added");
        }
        Commands::Show => {
            for block in &blockchain.blocks {
                println!("{:#?}", block);
            }
        }
        Commands::Validate => {
            println!("Blockchain valid? {}", blockchain.is_valid());
        }
    }
}
