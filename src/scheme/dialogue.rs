#![allow(clippy::useless_format)]
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

#[cfg(test)]
mod test {
    use super::*;
    use colored::Colorize;

    #[test]
    fn test_dialogue() {
        let dialogue = "Shakhzod Kudratov: Hello, Rust!\nRust: Hello, Shakhzod!"
            .to_string()
            .to_dialogue();

        assert_eq!(
            dialogue.to_string(),
            "\"Hello, Rust!\" - Shakhzod Kudratov\n\"Hello, Shakhzod!\" - Rust"
        );

        assert_eq!(
            dialogue.to_colorful_string(),
            format!(
                "{yb}{cnt1}{yb} - {auth1}\n{yb}{cnt2}{yb} - {auth2}",
                cnt1 = "Hello, Rust!".red(),
                cnt2 = "Hello, Shakhzod!".red(),
                auth1 = "Shakhzod Kudratov".yellow(),
                auth2 = "Rust".yellow(),
                yb = "\"".red()
            )
        );

        assert_eq!(
            dialogue.to_blockquote(),
            "<blockquote>\n<p><strong>Hello, Rust!</strong> - <i>Shakhzod Kudratov</i></p>\n<p><strong>Hello, Shakhzod!</strong> - <i>Rust</i></p>\n</blockquote>\n"
        );

        assert_eq!(
            dialogue.to_paragraph(),
            "<p>\n<p><strong>Hello, Rust!</strong> - <i>Shakhzod Kudratov</i></p>\n<p><strong>Hello, Shakhzod!</strong> - <i>Rust</i></p>\n</p>\n"
        );
    }
}
