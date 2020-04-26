use serde::{Deserialize, Serialize};

use crate::alignment::*;
use crate::effect::*;
use crate::event::*;
use crate::fate::*;
use crate::input::*;
use crate::log::*;
use crate::objective::*;
use crate::phase::*;
use crate::state::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        for (faction, _) in &self.state.factions {
            if self.get_fate(faction) == Fate::Won {
                self.log.push(Event::Won(faction.clone()));
            }
        }

        self.log.push(Event::PhaseEnded(self.phase.clone()));
        self.phase = self.phase.next();
    }

    fn get_faction(self: &Self, player: &Player) -> Faction {
        for modifier in self.state.players[player].iter().rev() {
            match &modifier.effect {
                Effect::BelongsTo(faction) => return faction.clone(),
                _ => {}
            }
        }
        "None".to_string()
    }

    fn get_player_alignment(self: &Self, player: &Player) -> Alignment {
        self.get_faction_alignment(&self.get_faction(player))
    }

    fn get_faction_alignment(self: &Self, faction: &Faction) -> Alignment {
        self.state.factions[faction].alignment.clone()
    }

    fn get_fate(self: &Self, faction: &Faction) -> Fate {
        let state = &self.state.factions[faction];

        match &state.objective {
            Objective::Eliminate(alignment) => {
                if self.num_alignment_remaining(alignment) == 0 {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::EliminateFaction(faction) => {
                if self.num_members_remaining(faction) == 0 {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::Majority => {
                if 2 * self.num_members_remaining(faction) > self.num_players_remaining() {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::Survive => {
                if self.num_members_remaining(faction) == 0 {
                    Fate::Lost
                } else {
                    Fate::Winning
                }
            }
        }
    }

    fn is_alive(self: &Self, player: &Player) -> bool {
        for modifier in self.state.players[player].iter().rev() {
            match modifier.effect {
                Effect::Dead => return true,
                _ => {}
            }
        }
        true
    }

    fn num_alignment_remaining(self: &Self, alignment: &Alignment) -> i64 {
        let mut result = 0;
        for (player, _) in &self.state.players {
            if self.is_alive(player) && self.get_player_alignment(player) == *alignment {
                result += 1
            }
        }
        result
    }

    fn num_members_remaining(self: &Self, faction: &Faction) -> i64 {
        let mut result = 0;
        for (player, _) in &self.state.players {
            if self.is_alive(player) && self.get_faction(player) == *faction {
                result += 1
            }
        }
        result
    }

    fn num_players_remaining(self: &Self) -> i64 {
        let mut result = 0;
        for (player, _) in &self.state.players {
            if self.is_alive(player) {
                result += 1
            }
        }
        result
    }
}
