use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ability {
    kind: AbilityKind,
    phase: AbilityPhase,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AbilityKind {
    Investigate,
    Kill,
    Protect,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AbilityPhase {
    Day,
    Night,
}
