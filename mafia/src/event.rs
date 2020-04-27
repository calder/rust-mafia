use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Event {
    Died(Player),
    Input(Input),
    PhaseEnded(Phase),
    Used(Action),
    Won(Faction),
}
