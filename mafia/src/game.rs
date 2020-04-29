use rand::seq::SliceRandom;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::attr::*;
use crate::deadline::*;
use crate::event::*;
use crate::fate::*;
use crate::input::*;
use crate::log::*;
use crate::objective::*;
use crate::phase::*;
use crate::state::*;
use crate::util::*;
use crate::visibility::*;

type Rng = rand_xoshiro::Xoshiro256StarStar;

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
        self.log
            .push((Visibility::Moderator, Event::Input(input.clone())));

        match input {
            Input::AdvancePhase => self.resolve(),
            Input::Use(player, action) => match action {
                Action::Immediate(action) => {
                    self.resolve_action(player, action);
                }
                Action::Investigate(_) => {}
                Action::Kill(_) => {}
                Action::Protect(_) => {}
                Action::Order(minion, faction_action) => match &**faction_action {
                    Action::Immediate(a) => {
                        self.resolve_action(minion, &*a);
                    }
                    _ => {}
                },
                Action::Vote(target) => {
                    self.log.push((
                        Visibility::Moderator,
                        Event::VotedFor(player.clone(), target.clone()),
                    ));
                }
            },
        }
    }

    fn add_attr(self: &mut Self, player: &Player, attr: Attr) {
        self.state.players.get_mut(player).unwrap().push(attr);
    }

    fn get_attr<T, F: FnMut(&Attr) -> Option<T>>(
        self: &Self,
        player: &Player,
        f: F,
        default: T,
    ) -> T {
        self.get_attrs(player).find_map(f).unwrap_or(default)
    }

    fn get_attr_sum<T: std::iter::Sum, F: FnMut(&Attr) -> Option<T>>(
        self: &Self,
        player: &Player,
        f: F,
    ) -> T {
        self.get_attrs(player).filter_map(f).sum()
    }

    fn get_attrs(self: &Self, player: &Player) -> std::iter::Rev<std::slice::Iter<Attr>> {
        self.state
            .players
            .get(player)
            .expect(&format!("No such player: {:?}", player))
            .iter()
            .rev()
    }

    fn get_faction(self: &Self, player: &Player) -> Faction {
        self.get_attrs(player)
            .find_map(|a| a.get_faction())
            .expect(&format!("Player does not have a faction: {:?}", player))
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

    fn get_living_players(self: &Self) -> Vec<Player> {
        self.state
            .players
            .keys()
            .filter(|p| self.is_alive(p))
            .map(|p| p.clone())
            .collect()
    }

    fn get_plan(self: &Self) -> Plan {
        let mut acted = Set::new();
        let mut plan = Plan::new();
        for (_visibility, event) in self.log.iter().rev() {
            match event {
                Event::PhaseEnded(_) => {
                    break;
                }
                Event::Input(Input::Use(_, Action::Immediate(_))) => {}
                Event::Input(Input::Use(player, action)) => {
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

    fn get_rng(self: &mut Self) -> Rng {
        self.state.seed += 1;
        Rng::seed_from_u64(self.state.seed)
    }

    fn is_alive(self: &Self, player: &Player) -> bool {
        self.get_attr(player, |a| a.is_alive(), true)
    }

    fn is_protected(self: &Self, player: &Player) -> bool {
        self.get_attr(player, |a| a.is_protected(), false)
    }

    fn make_dead(self: &mut Self, player: &Player) {
        self.state.players.get_mut(player).unwrap().push(Attr::Dead);
        self.log
            .push((Visibility::Moderator, Event::Died(player.clone())));
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

    fn num_votes_for(self: &Self, player: &Player) -> i64 {
        self.get_attr_sum(player, |a| a.num_votes())
    }

    fn resolve(self: &mut Self) {
        let plan = self.get_plan();

        // Resolve actions.
        for (player, action) in &plan {
            self.resolve_action(player, action);
        }

        // Resolve elimination.
        if let Phase::Day(_) = self.phase {
            let mut queue = self.get_living_players();
            let mut rng = self.get_rng();
            queue.shuffle(&mut rng);
            queue.sort_by_cached_key(|p| -self.num_votes_for(p));
            if let Some(target) = queue.first() {
                self.make_dead(target);
            }
        }

        // Expire effects.
        self.state.players = self
            .state
            .players
            .iter()
            .map(|(player, modifiers)| {
                (
                    player.clone(),
                    modifiers.iter().filter_map(|m| m.next_phase()).collect(),
                )
            })
            .collect();

        // Evaluate win conditions.
        for (faction, _) in &self.state.factions {
            if self.get_fate(faction) == Fate::Won {
                self.log
                    .push((Visibility::Moderator, Event::Won(faction.clone())));
            }
        }

        // Advance phase.
        self.log
            .push((Visibility::Moderator, Event::PhaseEnded(self.phase.clone())));
        self.phase = self.phase.next();
    }

    fn resolve_action(self: &mut Self, _player: &Player, action: &Action) {
        match action {
            Action::Immediate(_) => {}
            Action::Kill(target) => {
                if self.is_alive(target) && !self.is_protected(target) {
                    self.make_dead(target);
                }
            }
            Action::Investigate(target) => {
                let result = self.get_player_alignment(target);
                self.log.push((
                    Visibility::Moderator,
                    Event::FoundAlignment(target.clone(), result.clone()),
                ));
            }
            Action::Order(minion, faction_action) => self.resolve_action(minion, faction_action),
            Action::Protect(target) => {
                self.add_attr(
                    target,
                    Attr::Temporarily(Box::new(Attr::Protected), Deadline::Phases(1)),
                );
            }
            Action::Vote(target) => {
                self.add_attr(
                    target,
                    Attr::Temporarily(Box::new(Attr::ReceivedVotes(1)), Deadline::Phases(1)),
                );
            }
        }
    }
}
