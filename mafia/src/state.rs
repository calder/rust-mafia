use crate::modifier::*;
use crate::util::*;

pub type PlayerState = Vec<Modifier>;
pub type State = Map<Player, PlayerState>;
