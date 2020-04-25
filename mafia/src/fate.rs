use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Fate {
    // Would lose if the game were to end right now.
    Losing,

    // Has irrevocably lost the game (baring time travel).
    Lost,

    // Would win if the game were to end right now.
    Winning,

    // Has won the game, ending it.
    Won,
}
