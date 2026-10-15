use clap::{Parser, Subcommand};
use moive_importer::convert_txt_to_json;
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(version, about = "Convert movie lists to JSON (crate name: moive_importer)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(value_name = "INPUT.txt")]
        input: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert { input } => {
            let saved_path = convert_txt_to_json(&input)?;
            println!("Saved path: {}", saved_path.display());
        }
    }
    Ok(())
}