use serde::{Deserialize, Serialize};

use crate::ability::*;
use crate::deadline::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Attr {
    Member(Faction),
    Can(Ability),
    Dead,
    Poisoned(Deadline),
    Protected,
    ReceivedVotes(i64),
    Temporarily(Box<Attr>, Deadline),
}

impl Attr {
    pub fn get_faction(self: &Self) -> Option<Faction> {
        match self {
            Self::Member(f) => Some(f.clone()),
            Self::Temporarily(a, _) => a.get_faction(),
            _ => None,
        }
    }

    pub fn is_alive(self: &Self) -> Option<bool> {
        match self {
            Self::Dead => Some(false),
            Self::Temporarily(a, _) => a.is_alive(),
            _ => None,
        }
    }

    pub fn is_protected(self: &Self) -> Option<bool> {
        match self {
            Self::Protected => Some(true),
            Self::Temporarily(a, _) => a.is_protected(),
            _ => None,
        }
    }

    pub fn next_phase(self: &Self) -> Option<Self> {
        match self {
            Self::Temporarily(a, d) => match d.next_phase() {
                Some(d) => Some(Self::Temporarily(a.clone(), d)),
                None => None,
            },
            a => Some(a.clone()),
        }
    }

    pub fn num_votes(self: &Self) -> Option<i64> {
        match self {
            Attr::ReceivedVotes(n) => Some(*n),
            Self::Temporarily(a, _) => a.num_votes(),
            _ => None,
        }
    }
}
