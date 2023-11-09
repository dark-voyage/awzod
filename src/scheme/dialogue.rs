use serde::{Deserialize, Serialize};
use super::quotes::Quote;

#[derive(Serialize, Deserialize)]
pub struct Dialogue {
    series: Vec<Quote>
}