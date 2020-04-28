use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::deadline::*;
use crate::effect::*;
use crate::event::*;
use crate::fate::*;
use crate::input::*;
use crate::log::*;
use crate::modifier::*;
use crate::objective::*;
use crate::phase::*;
use crate::state::*;
use crate::util::*;

type Plan = Vec<(Player, Action)>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Game {
    pub start: State,
    pub state: State,
    pub phase: Phase,
    pub log: Log,
}

impl Game {
    pub fn new() -> Self {
        Self::new_from_state(State::new())
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
            Input::Plan(_, _) => { /* Do nothing until phase end. */ }
            Input::Use(_, _) => { /* PLACEHOLDER */ }
        }
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
                if self.num_living_alignment(alignment) == 0 {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::EliminateFaction(faction) => {
                if self.num_living_members(faction) == 0 {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::Majority => {
                if 2 * self.num_living_members(faction) > self.num_living_players() {
                    Fate::Won
                } else {
                    Fate::Losing
                }
            }
            Objective::Survive => {
                if self.num_living_members(faction) == 0 {
                    Fate::Lost
                } else {
                    Fate::Winning
                }
            }
        }
    }

    fn get_plan(self: &Self) -> Plan {
        let mut acted = Set::new();
        let mut plan = Plan::new();
        for event in self.log.iter().rev() {
            match event {
                Event::PhaseEnded(_) => {
                    break;
                }
                Event::Input(Input::Plan(player, action)) => {
                    if !acted.contains(player) {
                        // TODO: Check action validity.
                        plan.push((player.clone(), action.clone()));
                        acted.insert(player);
                    }
                }
                _ => {}
            }
        }
        plan.reverse();
        plan.sort_by_key(|(_, a)| a.order());
        plan
    }

    fn get_living_players(self: &Self) -> Vec<&Player> {
        self.state
            .players
            .keys()
            .filter(|p| self.is_alive(p))
            .collect()
    }

    fn is_alive(self: &Self, player: &Player) -> bool {
        for modifier in self.state.players[player].iter().rev() {
            match modifier.effect {
                Effect::Dead => return false,
                _ => {}
            }
        }
        true
    }

    fn is_protected(self: &Self, player: &Player) -> bool {
        for modifier in self.state.players[player].iter().rev() {
            match modifier.effect {
                Effect::Protected => return true,
                _ => {}
            }
        }
        false
    }

    fn num_living_alignment(self: &Self, alignment: &Alignment) -> usize {
        self.get_living_players()
            .iter()
            .filter(|p| self.get_player_alignment(p) == *alignment)
            .count()
    }

    fn num_living_members(self: &Self, faction: &Faction) -> usize {
        self.get_living_players()
            .iter()
            .filter(|p| self.get_faction(p) == *faction)
            .count()
    }

    fn num_living_players(self: &Self) -> usize {
        self.get_living_players().len()
    }

    fn resolve(self: &mut Self) {
        // Resolve actions.
        for (player, action) in &self.get_plan() {
            self.resolve_action(player, action);
        }

        // Evaluate win conditions.
        for (faction, _) in &self.state.factions {
            if self.get_fate(faction) == Fate::Won {
                self.log.push(Event::Won(faction.clone()));
            }
        }

        // TODO: Expire effects.

        // Advance phase.
        self.log.push(Event::PhaseEnded(self.phase.clone()));
        self.phase = self.phase.next();
    }

    fn resolve_action(self: &mut Self, _player: &Player, action: &Action) {
        match action {
            Action::Kill(target) => {
                if self.is_alive(target) && !self.is_protected(target) {
                    self.state
                        .players
                        .get_mut(target)
                        .unwrap()
                        .push(Modifier::new(Effect::Dead));
                    self.log.push(Event::Died(target.clone()));
                }
            }
            Action::Investigate(target) => {
                let result = self.get_player_alignment(target);
                self.log
                    .push(Event::FoundAlignment(target.clone(), result.clone()));
            }
            Action::Order(player, faction_action) => self.resolve_action(player, faction_action),
            Action::Protect(player) => {
                self.state
                    .players
                    .get_mut(player)
                    .unwrap()
                    .push(Modifier::new_with_deadline(
                        Effect::Protected,
                        Deadline::Nights(1),
                    ));
            }
        }
    }
}
