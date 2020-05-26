use serde::{Deserialize, Serialize};

/// Faction membership status.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Membership {
    /// Members don't know other members.
    Hidden,

    /// Membership is visible to all faction members.
    Visible,
}
