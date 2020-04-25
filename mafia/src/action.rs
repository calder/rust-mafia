use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}
