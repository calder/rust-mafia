use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Attr {
    Dead,
    Has(Action),
    Member(Faction),
    Phases(u64, Box<Attr>),
    Poisoned(u64),
    Bulletproof,
    ReceivedVotes(i64),
}

impl Attr {
    pub fn get_action(self: &Self) -> Option<Action> {
        match self {
            Self::Has(a) => Some(a.clone()),
            Self::Phases(_, a) => a.get_action(),
            _ => None,
        }
    }

    pub fn get_faction(self: &Self) -> Option<Faction> {
        match self {
            Self::Member(f) => Some(f.clone()),
            Self::Phases(_, a) => a.get_faction(),
            _ => None,
        }
    }

    pub fn is_alive(self: &Self) -> Option<bool> {
        match self {
            Self::Dead => Some(false),
            Self::Phases(_, a) => a.is_alive(),
            _ => None,
        }
    }

    pub fn is_bulletproof(self: &Self) -> Option<bool> {
        match self {
            Self::Bulletproof => Some(true),
            Self::Phases(_, a) => a.is_bulletproof(),
            _ => None,
        }
    }

    pub fn next_phase(self: &Self) -> Option<Self> {
        match self {
            Self::Phases(1, _) => None,
            Self::Phases(n, a) => Some(Self::Phases(n - 1, a.clone())),
            _ => Some(self.clone()),
        }
    }

    pub fn num_votes(self: &Self) -> Option<i64> {
        match self {
            Attr::ReceivedVotes(n) => Some(*n),
            Self::Phases(_, a) => a.num_votes(),
            _ => None,
        }
    }
}
