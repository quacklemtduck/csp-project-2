use std::{io, sync::Arc};

use crate::chunky_barrier::chunky_mergesort_barrier;
use clap::{command, Parser, Subcommand};
use partitioning::independent_output;
pub mod chunky_barrier;
pub mod partitioning;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    BenchSortData {
        file_path: String
    },
    BenchPartitioningData {
        file_path: String
    },
    #[command(arg_required_else_help = true)]
    Mergesort {
        num_threads: usize,
        threshold: usize,
        file_path: String
    },
    #[command(arg_required_else_help = true)]
    Partitioning {
        num_threads: i32,
        num_hash_bits: i32,
        file_path: String
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::BenchSortData { file_path } => {  
            let _elements = chunky_barrier::read_data(&file_path);
        },
        Commands::BenchPartitioningData { file_path } => {
            let _elements = partitioning::read_data(&file_path);
        },
        Commands::Mergesort { num_threads, threshold, file_path } => {
            let mut elements = chunky_barrier::read_data(&file_path);
            chunky_mergesort_barrier(&mut elements, num_threads, threshold);
        },
        Commands::Partitioning { num_threads, num_hash_bits, file_path} => {
            let elements = partitioning::read_data(&file_path);
            independent_output(Arc::new(elements), num_threads, num_hash_bits);
        }
    }


    Ok(())
}
