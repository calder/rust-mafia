use serde::{Deserialize, Serialize};

pub type Map<K, V> = std::collections::BTreeMap<K, V>;
pub type Set<T> = std::collections::BTreeSet<T>;

pub type Log = Vec<Event>;
pub type Inputs = Vec<Input>;
pub type Player = String;
pub type State = Map<Player, PlayerState>;

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
            Input::AdvancePhase => {
                self.log.push(Event::PhaseEnded(self.phase.clone()));
                self.phase = self.phase.next();
            }
            Input::Use(_, _) => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    modifiers: Vec<Modifier>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    effect: Effect,
    expiration: Deadline,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    Has(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Deadline {
    Never,
    Days(i64),
    Nights(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Phase {
    Day(i64),
    Night(i64),
}

impl Phase {
    pub fn prev(self: &Self) -> Self {
        match self {
            Phase::Day(n) => Phase::Night(n - 1),
            Phase::Night(n) => Phase::Day(*n),
        }
    }

    pub fn next(self: &Self) -> Self {
        match self {
            Phase::Day(n) => Phase::Night(*n),
            Phase::Night(n) => Phase::Day(n + 1),
        }
    }

    pub fn seq(self: &Self) -> i64 {
        match self {
            Self::Day(n) => 2 * n - 1,
            Self::Night(n) => 2 * n,
        }
    }

    pub fn id(self: &Self) -> String {
        match self {
            Self::Day(n) => format!("day{}", n),
            Self::Night(n) => format!("night{}", n),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Died(Player),
    Input(Input),
    Planned(Player, Action),
    PhaseEnded(Phase),
    Used(Player, Action),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ability {
    Investigate,
    Kill,
    Protect,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Input {
    AdvancePhase,
    Use(Player, Action),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
