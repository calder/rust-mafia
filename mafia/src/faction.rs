use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::alignment::*;
use crate::objective::*;
use crate::util::*;

pub type Factions = Map<Faction, FactionState>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FactionState {
    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    pub abilities: Vec<Ability>,

    pub alignment: Alignment,

    pub membership: Membership,

    pub objective: Objective,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Membership {
    Hidden,
    Visible,
}
