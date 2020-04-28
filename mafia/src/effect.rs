use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::deadline::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Effect {
    Member(Faction),
    Can(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
    ReceivedVotes(i64),
}
