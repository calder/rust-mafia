use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Ability {
    /// Investigate a player's alignment.
    Investigate,

    /// Kill a player.
    Kill,

    /// Protect a player from kills.
    Protect,
}
