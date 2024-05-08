use std::{fs, io};

use crate::chunky_barrier::chunky_mergesort_barrier;
use clap::{command, Parser, Subcommand};
pub mod chunky_barrier;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    BenchData {},
    #[command(arg_required_else_help = true)]
    Run {
        num_threads: usize,
        threshold: usize
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::BenchData {} => {  
            let _elements = read_data("../merge.out");
        },
        Commands::Run { num_threads, threshold } => {
            let mut elements = read_data("../merge.out");
            chunky_mergesort_barrier(&mut elements, num_threads, threshold);
        }
    }


    Ok(())
}

fn read_data(file_path: &str) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();
    let file = fs::read(file_path).unwrap();
    for bytes in file.chunks_exact(4) {
        let value =  u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        input.push(value);
    }
    input
}

fn is_sorted(elements: Vec<u32>) -> bool {
    for window in elements.windows(2) {
        let first = window[0];
        let second = window[1];
        if first > second {
            return false;
        }
    }
    true
}
