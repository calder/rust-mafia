type Map<K, V> = std::collections::BTreeMap<K, V>;
type Set<T> = std::collections::BTreeSet<T>;

type Player = String;

struct Game {
    players: Set<Player>,
    start: State,
    state: State,
    log: Log,
}

struct State {
    players: Map<Player, PlayerState>,
    phase: Phase,
}

struct PlayerState {
    modifiers: Vec<Modifier>,
}

struct Modifier {
    effect: Effect,
    expiration: Deadline,
}

enum Effect {
    Has(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
}

enum Deadline {
    Never,
    Days(i64),
    Nights(i64),
}

enum Phase {
    Day,
    Night,
}

struct Log {
    events: Vec<Event>,
}

enum Event {
    Planned(Player, Action),
    Used(Player, Action),
    Died(Player),
}

enum Action {
    Investigate(Player),
    Kill(Player),
    Protect(Player),
}

enum Ability {
    Investigate,
    Kill,
    Protect,
}

enum Command {
    Plan(Action),
    Use(Action),
}

#[cfg(test)]
mod test {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}
