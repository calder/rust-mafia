use std::io::Write;

use mafia::{Ability, Alignment, Attr, FactionState, Membership, Objective, Visibility};

use crate::util::KeyMap;

pub fn init(path: std::path::PathBuf, seed: Option<u64>) {
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut setup = mafia::State::new();
    setup.factions.insert(
        "Mafia".to_string(),
        FactionState {
            abilities: [Ability::Kill].to_vec(),
            alignment: Alignment::Evil,
            membership: Membership::Visible,
            objective: Objective::AchieveMajority,
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
            Attr::Has(Ability::Investigate),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Bob".to_string(),
        [Attr::Member("Town".to_string())].to_vec(),
    );
    setup.players.insert(
        "Charlie".to_string(),
        [
            Attr::Member("Town".to_string()),
            Attr::Has(Ability::Protect),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Eve".to_string(),
        [Attr::Member("Mafia".to_string())].to_vec(),
    );
    setup.players.insert(
        "Malory".to_string(),
        [Attr::Member("Mafia".to_string())].to_vec(),
    );
    setup.seed = seed.unwrap_or_else(rand::random);

    let mut keys = KeyMap::new();
    keys.insert("badpassword1".to_string(), Visibility::Moderator);
    keys.insert(
        "badpassword2".to_string(),
        Visibility::Player("Alice".to_string()),
    );
    keys.insert(
        "badpassword3".to_string(),
        Visibility::Player("Bob".to_string()),
    );
    keys.insert(
        "badpassword4".to_string(),
        Visibility::Player("Charlie".to_string()),
    );
    keys.insert(
        "badpassword5".to_string(),
        Visibility::Player("Eve".to_string()),
    );
    keys.insert(
        "badpassword6".to_string(),
        Visibility::Player("Malory".to_string()),
    );

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
