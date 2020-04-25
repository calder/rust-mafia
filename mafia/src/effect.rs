use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::deadline::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    Has(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
}
