use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Ability {
    Investigate,
    Kill,
    Protect,
    Vote,
}
