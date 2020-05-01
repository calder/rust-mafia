use serde::{Deserialize, Serialize};

use mafia::{Map, Player};

pub type KeyMap = Map<String, Entity>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Entity {
    Moderator,
    Player(Player),
}
