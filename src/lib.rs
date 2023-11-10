use std::path::PathBuf;

pub mod database;
pub mod input;
pub mod readme;
pub mod scheme;

pub use database::Database;
pub use readme::Readme;
pub use input::Input;

use clap::{Parser, Subcommand}; // Args

/// A CLI tool to manage golden quotes by Shakhzod Kudratov
#[derive(Debug, Parser)]
#[command(name = "awzod")]
#[command(about = "A CLI tool to manage golden quotes by Shakhzod Kudratov", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print out random quote or dialogue
    #[command(arg_required_else_help = false)]
    Random {
        /// Path to a json database
        #[arg(required = false)]
        path: Option<PathBuf>,
    },

    /// adds content to dump database
    #[command(arg_required_else_help = true)]
    Add {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<PathBuf>,
    },

    /// Render a markdown readme for this repo
    #[command(arg_required_else_help = true)]
    Render {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<PathBuf>,
    },
}
