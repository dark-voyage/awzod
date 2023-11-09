use crate::scheme::dialogue::ToDialogue;
use crate::scheme::quotes::ToQuote;
use asky::{Select, Text};
use std::process::exit;

use crate::scheme::category::Category;

pub struct Input {
    pub content: Vec<Category>,
}

pub enum Selection {
    Quote,
    Dialogue,
    Exit,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn ask_type(&mut self) -> Selection {
        let answer = Select::new("What you'd like to add?", vec!["Quote", "Dialogue", "Exit"])
            .prompt()
            .unwrap();

        match answer {
            "Quote" => Selection::Quote,
            "Dialogue" => Selection::Dialogue,
            "Exit" => Selection::Exit,
            _ => {
                eprintln!("Couldn't parse answer!");
                exit(1)
            }
        }
    }

    pub fn ask_quote(&mut self) {
        let answer = Text::new("Enter quote: ").prompt().unwrap();
        self.content.push(Category::Quote(answer.to_quote()));
    }

    pub fn ask_dialogue(&mut self) {
        loop {
            println!("Enter dialogue in format: \"Character: Dialogue\" or empty string to exit.");
            let answer = Text::new("Enter dialogue: ").prompt().unwrap();

            if answer.is_empty() {
                break;
            }

            self.content.push(Category::Dialogue(answer.to_dialogue()));
        }
    }

    pub fn ask(&mut self) {
        loop {
            match self.ask_type() {
                Selection::Quote => self.ask_quote(),
                Selection::Dialogue => self.ask_dialogue(),
                Selection::Exit => break,
            }
        }
    }
}
