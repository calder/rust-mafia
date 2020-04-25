use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ability {
    Investigate,
    Kill,
    Protect,
}
