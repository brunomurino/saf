//! Framework for creating plugins and running them via jobs defined in toml files.
//!
//! The goal of the crate is to make it very easy to implement the following pattern:
//! 1. Define a set of Action Structs that implement a common trait.
//! 1. Define Jobs (via TOML file) that instantiate an Action Struct with a set of arguments (also specified in the TOML file).
//! 1. Run a Job via the command line, allowing to further override the Action arguments.
//!
//! To start a new project with SAF, run the following
//! ```ignore
//! cargo init
//! ```
//! The add the following dependencies:
//! ```ignore
//! [dependencies]
//! saf = "0.0.2"
//! enum_dispatch = "0.3.13"
//! serde = { version = "1.0.205", features = ["derive"] }
//! serde_json = "1.0.122"
//! ```
//! Then add `cargo-run-bin` as a dev dependency to be able to run the SAF binary without installing SAF globally
//! ```ignore
//! cargo add cargo-run-bin --dev
//! ```
//! Now add this to your Cargo.toml
//! ```ignore
//! [package.metadata.bin]
//! saf = { version = "0.0.3", bins = ["main"] }
//! ```
//! Now you can call the SAF cli with
//! ```ignore
//! cargo bin saf <COMMAND>
//! ```
//! Now run `cargo bin saf init`
//! And finally run:
//! ```ignore
//! cargo run demo --job-config-repo __jobs
//! ```

pub mod job_config_file;

pub mod cli;
pub use cli::App;
