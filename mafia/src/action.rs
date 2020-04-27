use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    // Investigate a player's alignment.
    Investigate(Player, Player),

    // Kill a player.
    Kill(Player, Player),

    // Order a minion to perform an action.
    Order(Player, std::boxed::Box<Action>),

    // Protect a player from kills.
    Protect(Player, Player),
}

impl Action {
    pub fn player(self: &Self) -> &Player {
        match self {
            Self::Investigate(player, _target) => player,
            Self::Kill(player, _target) => player,
            Self::Order(player, _action) => player,
            Self::Protect(player, _target) => player,
        }
    }
}
