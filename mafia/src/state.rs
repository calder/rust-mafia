use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::modifier::*;
use crate::util::*;

pub type Factions = Map<Faction, FactionState>;
pub type Players = Map<Player, PlayerState>;
pub type PlayerState = Vec<Modifier>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct State {
    factions: Factions,
    players: Players,
}

impl State {
    pub fn new() -> Self {
        State {
            factions: Factions::new(),
            players: Players::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FactionState {
    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    abilities: Vec<Ability>,

    membership: Membership,

    objective: Objective,
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
