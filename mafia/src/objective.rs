use serde::{Deserialize, Serialize};

use crate::alignment::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Objective {
    Eliminate(Alignment),
    EliminateFaction(Faction),
    Majority,
    Survive,
}
