#![forbid(unsafe_code)]

pub mod build_metadata;
pub mod parser;
pub mod runner;

pub use runner::{main_entry, run_args, CliRunResult};
