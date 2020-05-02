use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Action {
    // Immediately resolve an action.
    Immediate(std::boxed::Box<Action>),

    // Investigate a player's alignment.
    Investigate(Player),

    // Kill a player.
    Kill(Player),

    // Order a minion to perform an action.
    Order(Player, std::boxed::Box<Action>),

    // Protect a player from kills.
    Protect(Player),

    // Vote to eliminate a player from the game.
    Vote(Player),
}

impl Action {
    pub fn order(self: &Self) -> usize {
        match self {
            Self::Immediate(_) => 0,
            Self::Investigate(_) => 1,
            Self::Protect(_) => 2,
            Self::Kill(_) => 1000,
            Self::Order(_, a) => a.order(),
            Self::Vote(_) => 1000,
        }
    }
}
