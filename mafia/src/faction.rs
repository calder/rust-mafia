use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::util::*;

pub type Factions = Map<Faction, FactionState>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FactionState {
    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    pub abilities: Vec<Ability>,

    pub membership: Membership,

    pub objective: Objective,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Membership {
    Hidden,
    Visible,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Objective {
    EliminateMafia,
    AchieveMajority,
}
