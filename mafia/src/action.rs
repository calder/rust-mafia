use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Faction(std::boxed::Box<Action>),
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}
