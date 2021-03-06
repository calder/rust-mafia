//! Rust backend for the classic party game Mafia.

// Re-enable when ready to start documenting.
// #![error(missing_docs)]

mod action;
mod alignment;
mod attr;
mod event;
mod faction;
mod fate;
mod game;
mod input;
mod log;
mod membership;
mod objective;
mod phase;
mod player;
mod state;
mod util;
mod visibility;

pub use crate::action::*;
pub use crate::alignment::*;
pub use crate::attr::*;
pub use crate::event::*;
pub use crate::faction::*;
pub use crate::fate::*;
pub use crate::game::*;
pub use crate::input::*;
pub use crate::log::*;
pub use crate::membership::*;
pub use crate::objective::*;
pub use crate::phase::*;
pub use crate::player::*;
pub use crate::state::*;
pub use crate::util::*;
pub use crate::visibility::*;
