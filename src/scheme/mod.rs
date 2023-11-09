pub mod quotes;
pub mod dialogue;

use crate::scheme::{
    quotes::Quote,
    dialogue::Dialogue
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Category {
    Quote(Quote),
    Dialogue(Dialogue)
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    category: Category
}