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
//! ```ignore
//! cargo add cargo-run-bin --dev
//! ```
//! Now add this to your Cargo.toml
//! ```ignore
//! [package.metadata.bin]
//! saf = { version = "0.0.1" }
//! ```
//! Now you can call the SAF cli with
//! ```ignore
//! cargo bin saf <COMMAND>
//! ```
//! Now run `saf init`

pub mod job_config_file;

pub mod cli;
pub use cli::App;
