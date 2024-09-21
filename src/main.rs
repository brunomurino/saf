use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// init a new project
    Init {
        /// Define name for new project
        #[arg(short, long)]
        name: String,
    },
}

fn create_file(filename: &String, content_as_bytes: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content_as_bytes)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let sample_main = include_str!("../sample_project/main.rs");
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init { name }) => {
            create_file(name, sample_main.as_bytes())?;
            Ok(())
        }
        None => Ok(()),
    }
}
