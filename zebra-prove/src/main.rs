use clap::{Parser, Subcommand};
use log::info;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use std::path::PathBuf;

mod proof_utils;
use proof_utils::load_and_print_proof;
use crate::proof_utils::{
    load_proof_from_compressed_bincode,
};

#[derive(Parser)]
#[command(name = "gpp")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Load a proof file and print its output
    ProofOutput {
        /// Path to the proof file (either .json or .bz format)
        #[arg(short, long)]
        proof_file: PathBuf,
    },
    BinaryToCairoSerde {
        /// Path to the proof file (.bz format)
        #[arg(short, long)]
        proof_file: PathBuf,

        /// Output path
        #[arg(short, long)]
        output_path: PathBuf,
    },
    JsonToBinary {
        /// Path to the proof file (.json format)
        #[arg(short, long)]
        proof_file: PathBuf,

        /// Output path
        #[arg(short, long)]
        output_path: PathBuf,
    },
    CairoSerdeToBinary {
        /// Path to the proof file (.json format)
        #[arg(short, long)]
        proof_file: PathBuf,

        /// Output path
        #[arg(short, long)]
        output_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    match cli.command {
        Commands::ProofOutput { proof_file } => {
            info!("Loading proof from: {}", proof_file.display());
            load_and_print_proof(&proof_file)?;
        },
        Commands::BinaryToCairoSerde {
            proof_file,
            output_path,
        } => {
            info!("=== Convert Proof ===");
            let proof = load_proof_from_compressed_bincode(&proof_file)?;
            cairo_air::utils::serialize_proof_to_file::<Blake2sMerkleChannel>(&proof, output_path, cairo_air::utils::ProofFormat::CairoSerde)?;
            info!("Proof converted successfully");
        },
        Commands::JsonToBinary {
            proof_file,
            output_path,
        } => {
            info!("=== Convert Proof to Binary ===");
            let proof = proof_utils::load_proof_from_file(&proof_file)?;
            proof_utils::serialize_proof_to_file(&proof, &output_path)?;
            info!("Proof converted successfully");
        }
        Commands::CairoSerdeToBinary {
            proof_file,
            output_path,
        } => {
            info!("=== Convert Proof to Binary ===");
            let proof = proof_utils::load_proof_from_cairo_serde(&proof_file)?;
            proof_utils::serialize_proof_to_file(&proof, &output_path)?;
            info!("Proof converted successfully");
        }
    }

    Ok(())
}
