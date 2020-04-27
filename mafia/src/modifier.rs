use serde::{Deserialize, Serialize};

use crate::deadline::*;
use crate::effect::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Modifier {
    pub effect: Effect,

    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    pub deadline: Deadline,
}

impl Modifier {
    pub fn new(effect: Effect) -> Self {
        Self::new_with_deadline(effect, Deadline::Never)
    }

    pub fn new_with_deadline(effect: Effect, deadline: Deadline) -> Self {
        Modifier {
            effect: effect,
            deadline: deadline,
        }
    }
}
