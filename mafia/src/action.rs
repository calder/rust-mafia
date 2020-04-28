use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    // Investigate a player's alignment.
    Investigate(Player),

    // Kill a player.
    Kill(Player),

    // Order a minion to perform an action.
    Order(Player, std::boxed::Box<Action>),

    // Protect a player from kills.
    Protect(Player),
}

impl Action {
    pub fn order(self: &Self) -> usize {
        match self {
            Self::Investigate(_) => 0,
            Self::Protect(_) => 1,
            Self::Kill(_) => 1000,
            Self::Order(_, a) => a.order(),
        }
    }
}
