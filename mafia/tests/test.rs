mod util;

use mafia::*;

use util::save;

#[test]
fn test_new() {
    let mut mint = goldenfile::Mint::new("tests/test_new");

    let game = Game::new();
    save(&mut mint, "out.ron", &game);
}
