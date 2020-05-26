use serde::{Deserialize, Serialize};

use crate::action::*;
use crate::alignment::*;
use crate::membership::*;
use crate::objective::*;
use crate::phase::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Attr {
    /// Faction only: Moral alignment.
    Alignment(Alignment),

    /// Player only: Player is immune to normal kills.
    Bulletproof,

    /// Player only: Player is dead.
    Dead,

    /// Faction or player: Faction leader or player has the given action.
    Has(Action),

    /// Player only: Player belongs to the given faction.
    Member(
        /// Faction player belongs to.
        Faction,
        /// Rank within the faction. Lower is better.
        #[serde(default, skip_serializing_if = "IsDefault::is_default")]
        i64,
    ),

    /// Faction only: Whether faction members know each other's identities.
    Membership(Membership),

    /// Faction only: Win condition.
    Objective(Objective),

    /// An attribute that expires after a given number of phases.
    Phases(
        /// Number of phases this attribute lasts.
        u64,
        /// The temporary attribute.
        Box<Attr>,
    ),

    /// Player only: Player will die after a number of phases.
    Poisoned(u64),

    /// Player only: Player received a number of elimination votes.
    ReceivedVotes(i64),

    /// An attribute which can't be used for another phase
    Tapped(Box<Attr>),
}

impl Attr {
    /// Whether this attribute allows the given action to be taken.
    pub fn allows_action(self: &Self, phase: &Phase, actor: &str, action: &Action) -> bool {
        match self {
            Self::Has(a) => a.matches(phase, actor, action),
            Self::Phases(_, a) => a.allows_action(phase, actor, action),
            _ => false,
        }
    }

    pub fn get_action(self: &Self) -> Option<Action> {
        match self {
            Self::Has(a) => Some(a.clone()),
            Self::Phases(_, a) => a.get_action(),
            _ => None,
        }
    }

    pub fn get_alignment(self: &Self) -> Option<Alignment> {
        match self {
            Self::Alignment(a) => Some(a.clone()),
            Self::Phases(_, a) => a.get_alignment(),
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

    pub fn get_objective(self: &Self) -> Option<Objective> {
        match self {
            Self::Objective(o) => Some(o.clone()),
            Self::Phases(_, o) => o.get_objective(),
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
            Self::Tapped(a) => Some((**a).clone()),
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

    /// Return used version of the attribute.
    pub fn tap(self: &Self) -> Self {
        Attr::Tapped(Box::new(self.clone()))
    }
}
