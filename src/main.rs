// src/main.rs
/*
 * Main executable for QuantumChain
 */

use clap::Parser;
use quantumchain::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumChain - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
