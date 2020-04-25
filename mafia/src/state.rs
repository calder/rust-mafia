use serde::{Deserialize, Serialize};

use crate::faction::*;
use crate::player::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct State {
    pub factions: Factions,
    pub players: Players,
}

impl State {
    pub fn new() -> Self {
        State {
            factions: Factions::new(),
            players: Players::new(),
        }
    }
}
