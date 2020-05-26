use serde::{Deserialize, Serialize};

use crate::phase::*;
use crate::util::*;

/// An action a player can take or has taken in the game.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Action {
    /// An action which is used during the day instead of the night.
    Day(std::boxed::Box<Action>),

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
    pub fn matches(self: &Self, phase: &Phase, actor: &str, action: &Action) -> bool {
        match phase {
            Phase::Day(n) => match (self, action) {
                (Self::Day(a1), a2) => a1.matches(&Phase::Night(*n), actor, a2),
                _ => false,
            },

            Phase::Night(_) => match (self, action) {
                (Self::Immediate(a1), Self::Immediate(a2)) => a1.matches(phase, actor, a2),
                (Self::Investigate(pp), Self::Investigate(p)) => placeholder_matches(pp, actor, p),
                (Self::Protect(pp), Self::Protect(p)) => placeholder_matches(pp, actor, p),
                (Self::Kill(pp), Self::Kill(p)) => placeholder_matches(pp, actor, p),
                (Self::Order(pp, pa), Self::Order(p, a)) => {
                    placeholder_matches(pp, actor, p) && pa.matches(phase, p, a)
                }
                (Self::Vote(pp), Self::Vote(p)) => placeholder_matches(pp, actor, p),
                _ => false,
            },
        }
    }

    /// Return resolution priority. Lower numbers are resolved first.
    pub fn precedence(self: &Self) -> usize {
        match self {
            Self::Day(a) => a.precedence(),
            Self::Immediate(_) => 0,
            Self::Investigate(_) => 1,
            Self::Protect(_) => 2,
            Self::Kill(_) => 1000,
            Self::Order(_, a) => a.precedence(),
            Self::Vote(_) => 1000,
        }
    }
}

/// Return whether a target player matches a placeholder.
pub fn placeholder_matches(placeholder: &str, actor: &str, target: &str) -> bool {
    match placeholder {
        "$MEMBER" => true, // TODO: Fix.
        "$OTHER_PLAYER" => target != actor,
        "$PLAYER" => true,
        _ => target == placeholder,
    }
}
