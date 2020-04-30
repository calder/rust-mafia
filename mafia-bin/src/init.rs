use std::io::Write;

use mafia::{Ability, Alignment, Attr, FactionState, Membership, Objective};

use crate::auth::KeyMap;

pub fn init(path: std::path::PathBuf, seed: Option<u64>) {
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut setup = mafia::State::new();
    setup.factions.insert(
        "Mafia".to_string(),
        FactionState {
            abilities: [Ability::Kill].to_vec(),
            alignment: Alignment::Evil,
            membership: Membership::Visible,
            objective: Objective::Majority,
        },
    );
    setup.factions.insert(
        "Town".to_string(),
        FactionState {
            abilities: [].to_vec(),
            alignment: Alignment::Good,
            membership: Membership::Hidden,
            objective: Objective::Eliminate(Alignment::Evil),
        },
    );
    setup.players.insert(
        "Alice".to_string(),
        [
            Attr::Member("Town".to_string()),
            Attr::Can(Ability::Vote),
            Attr::Can(Ability::Investigate),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Bob".to_string(),
        [Attr::Member("Town".to_string()), Attr::Can(Ability::Vote)].to_vec(),
    );
    setup.players.insert(
        "Charlie".to_string(),
        [
            Attr::Member("Town".to_string()),
            Attr::Can(Ability::Vote),
            Attr::Can(Ability::Protect),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Eve".to_string(),
        [Attr::Member("Mafia".to_string()), Attr::Can(Ability::Vote)].to_vec(),
    );
    setup.players.insert(
        "Malory".to_string(),
        [Attr::Member("Mafia".to_string()), Attr::Can(Ability::Vote)].to_vec(),
    );
    setup.seed = seed.unwrap_or_else(rand::random);

    let mut keys = KeyMap::new();
    keys.insert("123".to_string(), "Alice".to_string());
    keys.insert("456".to_string(), "Bob".to_string());
    keys.insert("789".to_string(), "Charlie".to_string());

    init_file(path.join("setup.ron"), &setup);
    init_file(path.join("auth.ron"), &keys);
}

fn init_file<P: AsRef<std::path::Path>, T: serde::ser::Serialize>(path: P, value: &T) {
    if path.as_ref().exists() {
        warn!(
            "{} already exists. Refusing to overwrite it.",
            path.as_ref().to_str().unwrap()
        )
    } else {
        let mut file = std::fs::File::create(path).unwrap();
        let config = ron::ser::PrettyConfig::default();
        let value_str = ron::ser::to_string_pretty(&value, config).unwrap();
        file.write(value_str.as_bytes()).unwrap();
    }
}
