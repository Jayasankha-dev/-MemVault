use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

mod entropy;
mod scanner;
mod dumper;

#[derive(Parser)]
#[command(name = "MemVault", version = "1.0", about = "High-Performance Memory Token Extractor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a running process for a specific byte pattern (e.g., 'eyJ' for JWT)
    Scan {
        #[arg(short, long)]
        pid: i32,
        #[arg(short, long)]
        pattern: String,
        #[arg(short, long, default_value_t = 5.8)]
        entropy: f64,
    },
    /// Dump the entire memory of a process to a raw binary file
    Dump {
        #[arg(short, long)]
        pid: i32,
        #[arg(short, long)]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { pid, pattern, entropy } => {
            println!(
                "{} Scanning Process PID: {} for pattern '{}' (Entropy Threshold: {})",
                "[*]".bold().blue(),
                pid,
                pattern,
                entropy
            );
            let results = scanner::scan_process_for_pattern(pid, pattern.as_bytes(), entropy)?;
            
            if results.is_empty() {
                println!("{} No high-entropy tokens found.", "[-]".bold().yellow());
            } else {
                println!("{} Found {} potential tokens:", "[+]".bold().green(), results.len());
                for token in results {
                    println!("  > {}", token.red());
                }
            }
        }
        Commands::Dump { pid, output } => {
            println!(
                "{} Dumping PID: {} to file: {}",
                "[*]".bold().blue(),
                pid,
                output
            );
            dumper::dump_full_process_memory(pid, &output)?;
        }
    }
    Ok(())
}