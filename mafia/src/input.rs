use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Input {
    // Resolve current phase and advance to next phase.
    EndPhase,

    // Use an action.
    Use(Player, Action),
}

pub type Inputs = Vec<Input>;
