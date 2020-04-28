use serde::{Deserialize, Serialize};

use crate::faction::*;
use crate::player::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct State {
    pub factions: Factions,
    pub players: Players,
    pub seed: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            factions: Factions::new(),
            players: Players::new(),
            seed: 0,
        }
    }
}
