use rand::seq::SliceRandom;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::attr::*;
use crate::event::*;
use crate::fate::*;
use crate::input::*;
use crate::log::*;
use crate::objective::*;
use crate::phase::*;
use crate::player::*;
use crate::state::*;
use crate::util::*;
use crate::visibility::*;

type Rng = rand_xoshiro::Xoshiro256StarStar;

type Plan = Vec<(Player, Action)>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
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
        let phase = Phase::Night(0);
        Game {
            start: state.clone(),
            state: state,
            phase: phase.clone(),
            log: [(Visibility::Public, Event::PhaseBegan(phase))].to_vec(),
        }
    }

    pub fn apply(self: &mut Self, input: &Input) -> &[(Visibility, Event)] {
        let log_start = self.log.len();
        self.log
            .push((Visibility::Moderator, Event::Input(input.clone())));

        match input {
            Input::EndPhase => self.resolve(),
            Input::Use(player, action) => match action {
                Action::Immediate(action) => {
                    self.resolve_action(player, action);
                }
                Action::Order(minion, faction_action) => match &**faction_action {
                    Action::Immediate(a) => {
                        self.resolve_action(minion, &*a);
                    }
                    _ => {}
                },
                Action::Vote(target) => {
                    self.log.push((
                        Visibility::Public,
                        Event::VotedFor(player.clone(), target.clone()),
                    ));
                }
                _ => {}
            },
        }

        &self.log[log_start..]
    }

    pub fn get_statuses(self: &Self) -> Map<Player, PlayerStatus> {
        self.state
            .players
            .keys()
            .map(|p| (p.clone(), self.get_status(p)))
            .collect()
    }

    fn add_attr(self: &mut Self, player: &Player, attr: Attr) {
        self.state.players.get_mut(player).unwrap().push(attr);
    }

    fn get_attr<T, F: FnMut(&Attr) -> Option<T>>(self: &Self, player: &Player, f: F) -> Option<T> {
        self.get_attrs(player).find_map(f)
    }

    fn get_attr_or<T, F: FnMut(&Attr) -> Option<T>>(
        self: &Self,
        player: &Player,
        f: F,
        default: T,
    ) -> T {
        self.get_attr(player, f).unwrap_or(default)
    }

    fn get_attr_sum<T: std::iter::Sum, F: FnMut(&Attr) -> Option<T>>(
        self: &Self,
        player: &Player,
        f: F,
    ) -> T {
        self.get_attrs(player).filter_map(f).sum()
    }

    /// Return a given player's attributes, most recent first.
    fn get_attrs(self: &Self, player: &Player) -> std::iter::Rev<std::slice::Iter<Attr>> {
        self.state
            .players
            .get(player)
            .expect(&format!("No such player: {:?}", player))
            .iter()
            .rev()
    }

    fn get_faction(self: &Self, player: &Player) -> Faction {
        self.get_faction_and_rank(player).0
    }

    fn get_faction_and_rank(self: &Self, player: &Player) -> (Faction, i64) {
        self.get_attrs(player)
            .find_map(|a| a.get_faction_and_rank())
            .expect(&format!("Player does not have a faction: {:?}", player))
    }

    fn get_faction_attr<T, F: FnMut(&Attr) -> Option<T>>(
        self: &Self,
        faction: &Faction,
        f: F,
    ) -> Option<T> {
        self.get_faction_attrs(faction).find_map(f)
    }

    /// Return a given player's attributes, most recent first.
    fn get_faction_attrs(self: &Self, faction: &Faction) -> std::iter::Rev<std::slice::Iter<Attr>> {
        self.state
            .factions
            .get(faction)
            .expect(&format!("No such faction: {:?}", faction))
            .iter()
            .rev()
    }

    fn get_player_alignment(self: &Self, player: &Player) -> Alignment {
        self.get_faction_alignment(&self.get_faction(player))
    }

    fn get_faction_alignment(self: &Self, faction: &Faction) -> Alignment {
        self.get_faction_attr(faction, |a| a.get_alignment())
            .expect(&format!(
                "Faction does not have an alignment: {:?}",
                faction
            ))
    }

    fn get_faction_objective(self: &Self, faction: &Faction) -> Objective {
        self.get_faction_attr(faction, |a| a.get_objective())
            .expect(&format!(
                "Faction does not have an objective: {:?}",
                faction
            ))
    }

    fn get_faction_fate(self: &Self, faction: &Faction) -> Fate {
        match &self.get_faction_objective(faction) {
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
            Objective::AchieveMajority => {
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

    fn get_leader(self: &Self, faction: &Faction) -> Player {
        self.get_members(faction)[0].clone()
    }

    fn get_living_players(self: &Self) -> Vec<Player> {
        self.state
            .players
            .keys()
            .filter(|p| self.is_alive(p))
            .map(|p| p.clone())
            .collect()
    }

    fn get_members(self: &Self, faction: &Faction) -> Vec<Player> {
        let mut members_with_rank: Vec<(Player, i64)> = self
            .get_living_players()
            .into_iter()
            .filter_map(|p| {
                let (f, r) = self.get_faction_and_rank(&p);
                if f == *faction {
                    Some((p, r))
                } else {
                    None
                }
            })
            .collect();
        members_with_rank.sort_by_key(|m| m.1);
        members_with_rank.into_iter().map(|(p, _)| p).collect()
    }

    /// Return inputs from the current phase.
    fn get_phase_inputs(self: &Self) -> Vec<Input> {
        let mut inputs = Vec::new();
        for event in self.log.iter().rev() {
            match event {
                (_, Event::PhaseBegan(_)) => {
                    break;
                }
                (_, Event::Input(input)) => {
                    inputs.push(input.clone());
                }
                _ => {}
            }
        }
        inputs.reverse();
        inputs
    }

    fn get_rng(self: &mut Self) -> Rng {
        self.state.seed += 1;
        Rng::seed_from_u64(self.state.seed)
    }

    fn get_status(self: &Self, player: &Player) -> PlayerStatus {
        PlayerStatus::alive(self.is_alive(player))
    }

    fn is_alive(self: &Self, player: &Player) -> bool {
        self.get_attr_or(player, |a| a.is_alive(), true)
    }

    fn is_bulletproof(self: &Self, player: &Player) -> bool {
        self.get_attr_or(player, |a| a.is_bulletproof(), false)
    }

    fn make_dead(self: &mut Self, player: &Player) {
        self.state.players.get_mut(player).unwrap().push(Attr::Dead);
        self.log
            .push((Visibility::Public, Event::Died(player.clone())));
    }

    fn num_living_alignment(self: &Self, alignment: &Alignment) -> usize {
        self.get_living_players()
            .iter()
            .filter(|p| self.get_player_alignment(p) == *alignment)
            .count()
    }

    fn num_living_members(self: &Self, faction: &Faction) -> usize {
        self.get_members(faction).len()
    }

    fn num_living_players(self: &Self) -> usize {
        self.get_living_players().len()
    }

    fn num_votes_for(self: &Self, player: &Player) -> i64 {
        self.get_attr_sum(player, |a| a.num_votes())
    }

    /// Resolve the current phase.
    fn resolve(self: &mut Self) {
        // Resolve actions.
        let plan = self.resolve_build_plan();
        for (player, action) in &plan {
            self.resolve_action(player, action);
        }

        // Resolve elimination.
        if let Phase::Day(_) = self.phase {
            let queue = self.get_living_players();
            let mut rng = self.get_rng();

            // Count votes.
            let mut queue: Vec<(i64, &Player)> =
                queue.iter().map(|p| (self.num_votes_for(p), p)).collect();

            // Sort the queue by decreasing vote count, randomizing ties.
            queue.shuffle(&mut rng);
            queue.sort_by_cached_key(|(v, _)| -v);

            // Kill the first player if they received positive votes.
            if let Some((votes, player)) = queue.first() {
                if *votes > 0 {
                    self.make_dead(player);
                }
            }
        }

        // Expire effects.
        for (_, attrs) in &mut self.state.players {
            *attrs = attrs.iter().filter_map(|m| m.next_phase()).collect();
        }
        for (_, attrs) in &mut self.state.factions {
            *attrs = attrs.iter().filter_map(|m| m.next_phase()).collect();
        }

        // Evaluate win conditions.
        for (faction, _) in &self.state.factions {
            if self.get_faction_fate(faction) == Fate::Won {
                self.log
                    .push((Visibility::Public, Event::Won(faction.clone())));
            }
        }

        // Advance phase.
        self.log
            .push((Visibility::Public, Event::PhaseEnded(self.phase.clone())));
        self.phase = self.phase.next();
        self.log
            .push((Visibility::Public, Event::PhaseBegan(self.phase.clone())));
    }

    /// Resolve the effects of a single action.
    fn resolve_action(self: &mut Self, player: &Player, action: &Action) {
        match action {
            Action::Kill(target) => {
                if self.is_alive(target) && !self.is_bulletproof(target) {
                    self.make_dead(target);
                }
            }
            Action::Investigate(target) => {
                let result = self.get_player_alignment(target);
                self.log.push((
                    Visibility::Player(player.clone()),
                    Event::FoundAlignment(target.clone(), result),
                ));
            }
            Action::Order(minion, faction_action) => self.resolve_action(minion, faction_action),
            Action::Protect(target) => {
                self.add_attr(target, Attr::Phases(1, Box::new(Attr::Bulletproof)));
            }
            Action::Vote(target) => {
                self.add_attr(target, Attr::Phases(1, Box::new(Attr::ReceivedVotes(1))));
            }
            _ => {}
        }
    }

    /// Build a plan of the actions to resolve and the order to resolve them in.
    fn resolve_build_plan(self: &mut Self) -> Plan {
        // Get valid actions.
        let mut plan = Plan::new();
        let mut log = Log::new();
        for input in self.get_phase_inputs().iter().rev() {
            match input {
                Input::Use(player, action) => {
                    if let Some(a) = self.resolve_get_action(player, action) {
                        plan.push((player.clone(), action.clone()));
                        *a = a.tap();
                        log.push((
                            Visibility::Moderator,
                            Event::Accepted(player.clone(), action.clone()),
                        ));
                    } else {
                        log.push((
                            Visibility::Moderator,
                            Event::Rejected(player.clone(), action.clone()),
                        ));
                    }
                }
                _ => {}
            }
        }

        // Sort actions by precedence, breaking ties by submission order.
        plan.reverse();
        plan.sort_by_key(|(_, a)| a.precedence());

        // Log accepted and rejected actions.
        log.reverse();
        self.log.append(&mut log);

        plan
    }

    /// Return the first placeholder action matching the given action, if any.
    fn resolve_get_action(self: &mut Self, player: &Player, action: &Action) -> Option<&mut Attr> {
        // Check faction actions.
        let mut led_factions = Set::new();
        for (faction, _state) in &self.state.factions {
            if self.get_leader(&faction) == *player {
                led_factions.insert(faction.clone());
            }
        }
        for (faction, attrs) in &mut self.state.factions {
            if led_factions.contains(faction) {
                for attr in attrs {
                    if attr.allows_action(&self.phase, player, action) {
                        return Some(attr);
                    }
                }
            }
        }

        // Check individual actions.
        for attr in self
            .state
            .players
            .get_mut(player)
            .expect(&format!("No such player: {:?}", player))
        {
            if let Some(a) = attr.get_action() {
                if a.matches(&self.phase, player, action) {
                    return Some(attr);
                }
            }
        }

        None
    }
}
