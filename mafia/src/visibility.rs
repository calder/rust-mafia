use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Visibility {
    Moderator,
    Player(Player),
    Public,
}
