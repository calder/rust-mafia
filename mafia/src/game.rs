use serde::{Deserialize, Serialize};

use crate::event::*;
use crate::input::*;
use crate::log::*;
use crate::phase::*;
use crate::state::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub start: State,
    pub state: State,
    pub phase: Phase,
    pub log: Log,
}

impl Game {
    pub fn new() -> Self {
        Game {
            start: State::new(),
            state: State::new(),
            phase: Phase::Night(0),
            log: Log::new(),
        }
    }

    pub fn new_from_state(state: State) -> Self {
        Game {
            start: state.clone(),
            state: state,
            phase: Phase::Night(0),
            log: Log::new(),
        }
    }

    pub fn apply(self: &mut Self, input: &Input) {
        self.log.push(Event::Input(input.clone()));

        match input {
            Input::AdvancePhase => self.resolve(),
            Input::Use(_, _) => {}
        }
    }

    fn resolve(self: &mut Self) {
        self.log.push(Event::PhaseEnded(self.phase.clone()));
        self.phase = self.phase.next();
    }
}
