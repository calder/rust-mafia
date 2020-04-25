use serde::{Deserialize, Serialize};

use crate::deadline::*;
use crate::effect::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    effect: Effect,
    expiration: Deadline,
}
