use awzod::scheme::Colorful;
use awzod::{Cli, Commands};
use clap::Parser;
use colored::*;
use std::process::exit;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Random { path } => {
            let database = awzod::database::Database::from_path_or_binary(path);
            let quote = database.content.random();

            print!("{}", quote.to_colorful_string());
        }
        Commands::Add { path } => {
            let mut database = match path {
                Some(c) => awzod::database::Database::from_file_or_new(c),
                None => {
                    eprintln!("{}", "No path for database given!".red());
                    exit(1)
                }
            };

            let mut input = awzod::input::Input::new();
            input.ask();

            database.content.merge(input.content);
            database.save(true).expect("Failed to save database!");
        }
        Commands::Render { path } => {
            let database = awzod::Database::from_path_or_binary(path);
            let readme = awzod::Readme::new();

            readme.render(database.content);
        }
    }
}
