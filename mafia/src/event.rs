use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

/// Event that occurred during a game.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    /// Action was accepted and resolved.
    Accepted(Player, Action),

    /// Player died.
    Died(Player),

    /// Investigation yielded alignment.
    FoundAlignment(Player, Alignment),

    /// Game received input.
    Input(Input),

    /// Phase started.
    PhaseBegan(Phase),

    /// Phase ended.
    PhaseEnded(Phase),

    /// Action was rejected because it was invalid or amended.
    Rejected(Player, Action),

    /// Player used action.
    Used(Player, Action),

    /// Player voted to eliminate another player.
    VotedFor(Player, Player),

    /// Faction won.
    Won(Faction),
}
