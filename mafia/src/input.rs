use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Input {
    AdvancePhase,
    Use(Player, Action),
}

pub type Inputs = Vec<Input>;
