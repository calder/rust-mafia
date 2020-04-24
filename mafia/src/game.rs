use serde::{Deserialize, Serialize};

type Map<K, V> = std::collections::BTreeMap<K, V>;
type Set<T> = std::collections::BTreeSet<T>;

type Player = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    players: Set<Player>,
    start: State,
    state: State,
    log: Log,
}

impl Game {
    fn new() -> Self {
        Game {
            players: Set::new(),
            start: State::new(),
            state: State::new(),
            log: Log::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    players: Map<Player, PlayerState>,
    phase: Phase,
}

impl State {
    fn new() -> Self {
        State {
            players: Map::new(),
            phase: Phase::Night(0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
    modifiers: Vec<Modifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modifier {
    effect: Effect,
    expiration: Deadline,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Effect {
    Has(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Deadline {
    Never,
    Days(i64),
    Nights(i64),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Phase {
    Day(i64),
    Night(i64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    events: Vec<Event>,
}

impl Log {
    fn new() -> Self {
        Log { events: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Planned(Player, Action),
    Used(Player, Action),
    Died(Player),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Ability {
    Investigate,
    Kill,
    Protect,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Plan(Action),
    Use(Action),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn placeholder() {
        let game = Game::new();
        println!("{:?}", ron::ser::to_string(&game));
        assert!(true);
    }
}
