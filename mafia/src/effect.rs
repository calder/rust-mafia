use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::deadline::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    BelongsTo(Faction),
    Has(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
}
