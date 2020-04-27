use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Input {
    AdvancePhase,
    Plan(Player, Action),
    Use(Player, Action),
}

pub type Inputs = Vec<Input>;
