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
