use serde::{Deserialize, Serialize};

use crate::attr::*;
use crate::util::*;

pub type Players = Map<Player, Vec<Attr>>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlayerStatus {
    /// Player is currently alive.
    Alive,

    /// Player is currently dead.
    Dead,
}

impl PlayerStatus {
    pub fn alive(alive: bool) -> Self {
        if alive {
            Self::Alive
        } else {
            Self::Dead
        }
    }

    pub fn is_alive(self: Self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
        }
    }
}
