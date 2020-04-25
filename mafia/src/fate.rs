use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Fate {
    Lost,
    Undecided,
    Won,
}
