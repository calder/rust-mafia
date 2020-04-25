use serde::{Deserialize, Serialize};

use crate::phase::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ability {
    kind: AbilityKind,
    phase: PhaseKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AbilityKind {
    Investigate,
    Kill,
    Protect,
}
