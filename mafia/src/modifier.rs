use serde::{Deserialize, Serialize};

use crate::deadline::*;
use crate::effect::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Modifier {
    effect: Effect,
    expires: Deadline,
}
