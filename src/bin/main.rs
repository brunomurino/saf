use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    Init {},
}

fn create_file(filename: &Path, content: &str) -> std::io::Result<()> {
    let prefix = filename.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let demo_toml = include_str!("../../__demo_jobs/demo.toml");
    let sample_main = include_str!("./sample_main.rs");

    let args = Args::parse();

    match &args.command {
        Some(Commands::Init {}) => {
            create_file(Path::new("src/main.rs"), sample_main)?;
            create_file(Path::new("__jobs/demo.toml"), demo_toml)?;
            Ok(())
        }
        None => Ok(()),
    }
}
