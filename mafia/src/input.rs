use serde::{Deserialize, Serialize};

use crate::action::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Input {
    // Resolve current phase and advance to next phase.
    AdvancePhase,

    // Plan normal action.
    Plan(Action),

    // Use instantaneous action.
    Use(Action),
}

pub type Inputs = Vec<Input>;
