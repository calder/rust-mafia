use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ability {
    kind: AbilityKind,

    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    phase: AbilityPhase,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AbilityKind {
    Investigate,
    Kill,
    Protect,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AbilityPhase {
    Day,
    Night,
}

impl Default for AbilityPhase {
    fn default() -> Self {
        AbilityPhase::Night
    }
}
