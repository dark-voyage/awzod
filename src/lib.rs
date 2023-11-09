use std::path::PathBuf;

pub mod scheme;
pub mod readme;
pub mod database;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// A CLI tool to manage golden quotes by Shakhzod Kudratov
#[derive(Debug, Parser)]
#[command(name = "awzod")]
#[command(about = "A CLI tool to manage golden quotes by Shakhzod Kudratov", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print out random quote or dialogue
    #[command(arg_required_else_help = true)]
    Random {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<Vec<PathBuf>>,
    },

    /// adds content to dump database
    #[command(arg_required_else_help = true)]
    Add {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<Vec<PathBuf>>,
    },

    /// Render a markdown readme for this repo
    #[command(arg_required_else_help = true)]
    Render {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<Vec<PathBuf>>,
    },

    /// Show a markdown file on readme
    #[command(arg_required_else_help = true)]
    Markdown {
        /// Path to a json database
        #[arg(required = true)]
        path: Option<Vec<PathBuf>>,
    }
}
