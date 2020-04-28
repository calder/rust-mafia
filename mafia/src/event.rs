use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Event {
    Died(Player),
    Input(Input),
    Investigated(Player, Player, Alignment),
    PhaseEnded(Phase),
    Used(Player, Action),
    Won(Faction),
}
