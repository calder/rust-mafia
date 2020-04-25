use serde::{Deserialize, Serialize};

use crate::deadline::*;
use crate::effect::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Modifier {
    pub effect: Effect,

    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    pub expires: Deadline,
}
