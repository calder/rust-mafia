use serde::{Deserialize, Serialize};

use crate::alignment::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Objective {
    Eliminate(Alignment),
    Majority,
    Survive,
}
