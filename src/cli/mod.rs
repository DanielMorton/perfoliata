pub mod args;
pub mod commands;
mod stats;

pub use args::{Cli, Commands, StatType};
pub use commands::execute_command;