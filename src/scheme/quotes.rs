use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quote {
    author: String,
    content: String
}
