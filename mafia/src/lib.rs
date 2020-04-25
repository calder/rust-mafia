//! Rust backend for the classic party game Mafia.

// Re-enable when ready to start documenting.
// #![error(missing_docs)]

mod ability;
mod action;
mod deadline;
mod effect;
mod event;
mod fate;
mod game;
mod input;
mod log;
mod modifier;
mod phase;
mod state;
mod util;

pub use crate::ability::*;
pub use crate::action::*;
pub use crate::deadline::*;
pub use crate::effect::*;
pub use crate::event::*;
pub use crate::fate::*;
pub use crate::game::*;
pub use crate::input::*;
pub use crate::log::*;
pub use crate::modifier::*;
pub use crate::phase::*;
pub use crate::state::*;
pub use crate::util::*;
