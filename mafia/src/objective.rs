use serde::{Deserialize, Serialize};

use crate::alignment::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Objective {
    /// Eliminate all players of a given alignment.
    Eliminate(Alignment),

    /// Eliminate all players of a given faction.
    EliminateFaction(Faction),

    /// Outnumber all other surviving players.
    AchieveMajority,

    /// Survive until the end of the game.
    Survive,
}
