use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    Died(Player),
    FoundAlignment(Player, Alignment),
    Input(Input),
    PhaseEnded(Phase),
    Used(Player, Action),
    VotedFor(Player, Player),
    Won(Faction),
}
