use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Input {
    // Resolve current phase and advance to next phase.
    AdvancePhase,

    // Plan normal action.
    Plan(Player, Action),

    // Use instantaneous action.
    Use(Player, Action),
}

pub type Inputs = Vec<Input>;
