use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}
