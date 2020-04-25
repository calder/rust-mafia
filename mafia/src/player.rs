use crate::modifier::*;
use crate::util::*;

pub type Players = Map<Player, PlayerState>;
pub type PlayerState = Vec<Modifier>;
