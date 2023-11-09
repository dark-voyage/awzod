#![allow(clippy::manual_split_once)]

use super::quotes::Quote;
use crate::scheme::{Colorful, Markdown};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Dialogue {
    series: Vec<Quote>,
}

pub trait ToDialogue {
    fn to_dialogue(&self) -> Dialogue;
}

impl Display for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for quote in &self.series {
            result.push_str(&format!("{}", quote));

            if quote != self.series.last().unwrap() {
                result.push('\n');
            }
        }
        write!(f, "{}", result)
    }
}

impl Colorful for Dialogue {
    fn to_colorful_string(&self) -> String {
        let mut result = String::new();
        for quote in &self.series {
            result.push_str(&format!("{}", quote.to_colorful_string()));

            if quote != self.series.last().unwrap() {
                result.push('\n');
            }
        }
        result
    }
}

impl Markdown for Dialogue {
    fn to_blockquote(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\n", "<blockquote>"));

        for quote in &self.series {
            result.push_str(&format!("{}\n", quote.to_paragraph()));
        }

        result.push_str(&format!("{}\n", "</blockquote>"));
        result
    }

    fn to_paragraph(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\n", "<p>"));

        for quote in &self.series {
            result.push_str(&format!("{}\n", quote.to_paragraph()));
        }

        result.push_str(&format!("{}\n", "</p>"));
        result
    }
}

// impl ToDialogue for String whereas format is "Character: Dialogue"
impl ToDialogue for String {
    fn to_dialogue(&self) -> Dialogue {
        let mut series = Vec::new();

        for line in self.lines() {
            let mut split = line.splitn(2, ": ");
            let character = split.next().unwrap();
            let dialogue = split.next().unwrap();
            series.push(Quote::new(character.to_string(), dialogue.to_string()));
        }

        Dialogue { series }
    }
}
