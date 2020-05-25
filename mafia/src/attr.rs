use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Attr {
    /// Player is immune to normal kills.
    Bulletproof,

    /// Player is dead.
    Dead,

    /// Player has an action.
    Has(Action),

    /// Player belongs to the given faction.
    Member(
        /// Faction player belongs to.
        Faction,
        /// Rank within the faction. Lower is better.
        #[serde(default, skip_serializing_if = "IsDefault::is_default")]
        i64,
    ),

    /// Player has an attribute that expires after a given number of phases.
    Phases(
        /// Number of phases this attribute lasts.
        u64,
        /// The temporary attribute.
        Box<Attr>,
    ),

    /// Player is poisoned and will die after a number of phases.
    Poisoned(u64),

    /// Player received a number of elimination votes.
    ReceivedVotes(i64),
}

impl Attr {
    pub fn get_action_mut(self: &mut Self) -> Option<&mut Action> {
        match self {
            Self::Has(a) => Some(a),
            Self::Phases(_, a) => a.get_action_mut(),
            _ => None,
        }
    }

    pub fn get_faction_and_rank(self: &Self) -> Option<(Faction, i64)> {
        match self {
            Self::Member(f, r) => Some((f.clone(), *r)),
            Self::Phases(_, a) => a.get_faction_and_rank(),
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
            Self::Has(action) => Some(Self::Has(action.next_phase())),
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
