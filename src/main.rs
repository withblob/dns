mod cli;
mod dns;
mod options;
mod config;

use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}