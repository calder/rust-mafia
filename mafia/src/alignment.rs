use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Alignment {
    Evil,
    Good,
    Neutral,
}
