use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Deadline {
    Never,
    Days(i64),
    Nights(i64),
}
