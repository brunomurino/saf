//! Framework for creating plugins and running them via jobs defined in toml files.
//!
//! The goal of the crate is to make it very easy to implement the following pattern:
//! 1. Define a set of Action Structs that implement a common trait.
//! 1. Define Jobs (via TOML file) that instantiate an Action Struct with a set of arguments (also specified in the TOML file).
//! 1. Run a Job via the command line, allowing to further override the Action arguments.
//!
//! # Your first Action
//! 1. To start off, we'll create a trait that we want our Action Structs to implement. Let's keep it simple for now:
//! ```
//! pub trait ActionRunner {
//!    fn run(&self) -> String;
//! }
//! ```
//! 2. Now we create an Action Struct that implements this trait:
//! ```
//! pub trait ActionRunner {
//!    fn run(&self) -> String;
//! }
//!
//! pub struct MyFirstAction {
//!     pub val: String,
//! }
//!
//! impl ActionRunner for MyFirstAction {
//!     fn run(&self) -> String {
//!         println!("Executing MyFirstAction");
//!         format!("Hello, {}!", self.val)
//!     }
//! }
//! ```
//! 3. and a Job specification in a TOML file:
//! ```toml
//! [action]
//! name = "MyFirstAction"
//!
//! [action.args]
//! val = "World"
//! ```
//!

pub mod job_config_file;

pub mod cli;
pub use cli::App;
