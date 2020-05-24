use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

/// Event that occurred during a game.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    /// Player died.
    Died(Player),

    /// Investigation yielded alignment.
    FoundAlignment(Player, Alignment),

    /// Action was ignored because it was invalid or amended.
    Ignored(Player, Action),

    /// Game received input.
    Input(Input),

    /// Phase started.
    PhaseBegan(Phase),

    /// Phase ended.
    PhaseEnded(Phase),

    /// Player used action.
    Used(Player, Action),

    /// Player voted to eliminate another player.
    VotedFor(Player, Player),

    /// Faction won.
    Won(Faction),
}
