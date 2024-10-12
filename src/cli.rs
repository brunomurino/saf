use clap::{value_parser, Parser};
use std::path::PathBuf;

#[derive(Parser)]
pub struct App {
    /// The name of the transfer to run
    pub scope: String,

    #[arg(short, long, default_value = "{}")]
    pub action_args: String,

    /// The folder containing transfers config files
    #[arg(short, long, value_parser=value_parser!(PathBuf))]
    pub job_config_repo: PathBuf,
}

impl App {
    pub fn get() -> Self {
        return App::parse();
    }
}
