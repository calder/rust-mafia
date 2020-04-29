use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Visibility {
    Faction(Faction),
    Moderator,
    Player(Player),
    Public,
}
