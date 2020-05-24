use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Action {
    /// Immediately resolve an action.
    Immediate(std::boxed::Box<Action>),

    /// Investigate a player's alignment.
    Investigate(Player),

    /// Kill a player.
    Kill(Player),

    /// Order a minion to perform an action.
    Order(Player, std::boxed::Box<Action>),

    /// Protect a player from kills.
    Protect(Player),

    /// Vote to eliminate a player from the game.
    Vote(Player),
}

impl Action {
    /// Return whether another action matches this one, respecting placeholders.
    pub fn matches(self: &Self, other: &Action, actor: &str) -> bool {
        match (self, other) {
            (Self::Immediate(a1), Self::Immediate(a2)) => a1.matches(a2, actor),
            (Self::Investigate(pp), Self::Investigate(p)) => placeholder_matches(pp, p, actor),
            (Self::Protect(pp), Self::Protect(p)) => placeholder_matches(pp, p, actor),
            (Self::Kill(pp), Self::Kill(p)) => placeholder_matches(pp, p, actor),
            (Self::Order(pp, pa), Self::Order(p, a)) => {
                placeholder_matches(pp, p, actor) && pa.matches(a, p)
            }
            (Self::Vote(pp), Self::Vote(p)) => placeholder_matches(pp, p, actor),
            _ => false,
        }
    }

    pub fn precedence(self: &Self) -> usize {
        match self {
            Self::Immediate(_) => 0,
            Self::Investigate(_) => 1,
            Self::Protect(_) => 2,
            Self::Kill(_) => 1000,
            Self::Order(_, a) => a.precedence(),
            Self::Vote(_) => 1000,
        }
    }
}

pub fn placeholder_matches(placeholder: &str, target: &str, actor: &str) -> bool {
    match placeholder {
        "$PLAYER" => true,
        "$OTHER_PLAYER" => target != actor,
        _ => target == placeholder,
    }
}
