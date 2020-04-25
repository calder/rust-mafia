use serde::{Deserialize, Serialize};

use crate::modifier::*;
use crate::util::*;

pub type State = Map<Player, PlayerState>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    modifiers: Vec<Modifier>,
}
