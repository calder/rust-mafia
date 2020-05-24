use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Attr {
    Dead,
    Has(Ability),
    Member(Faction),
    Phases(u64, Box<Attr>),
    Poisoned(u64),
    Bulletproof,
    ReceivedVotes(i64),
}

impl Attr {
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

    pub fn is_protected(self: &Self) -> Option<bool> {
        match self {
            Self::Bulletproof => Some(true),
            Self::Phases(_, a) => a.is_protected(),
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
