use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Objective {
    AchieveMajority,
    EliminateEvil,
    Survive,
}
