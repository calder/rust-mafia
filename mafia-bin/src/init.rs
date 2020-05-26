use std::io::Write;

use mafia::{Action, Alignment, Attr, Membership, Objective, Visibility};

use crate::util::KeyMap;

pub fn init(path: std::path::PathBuf, seed: Option<u64>) {
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut setup = mafia::State::new();
    setup.factions.insert(
        "Mafia".to_string(),
        [
            Attr::Has(Action::Order(
                "$MEMBER".to_string(),
                Box::new(Action::Kill("$PLAYER".to_string())),
            )),
            Attr::Alignment(Alignment::Evil),
            Attr::Membership(Membership::Visible),
            Attr::Objective(Objective::AchieveMajority),
        ]
        .to_vec(),
    );
    setup.factions.insert(
        "Town".to_string(),
        [
            Attr::Alignment(Alignment::Good),
            Attr::Membership(Membership::Hidden),
            Attr::Objective(Objective::Eliminate(Alignment::Evil)),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Alice".to_string(),
        [
            Attr::Member("Town".to_string(), 0),
            Attr::Has(Action::Investigate("$PLAYER".to_string())),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Bob".to_string(),
        [Attr::Member("Town".to_string(), 0)].to_vec(),
    );
    setup.players.insert(
        "Charlie".to_string(),
        [
            Attr::Member("Town".to_string(), 0),
            Attr::Has(Action::Protect("$OTHER_PLAYER".to_string())),
        ]
        .to_vec(),
    );
    setup.players.insert(
        "Eve".to_string(),
        [Attr::Member("Mafia".to_string(), 1)].to_vec(),
    );
    setup.players.insert(
        "Malory".to_string(),
        [Attr::Member("Mafia".to_string(), 2)].to_vec(),
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
