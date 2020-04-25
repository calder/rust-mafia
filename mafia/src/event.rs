use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::input::*;
use crate::phase::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Died(Player),
    Input(Input),
    Planned(Player, Action),
    PhaseEnded(Phase),
    Used(Player, Action),
}
