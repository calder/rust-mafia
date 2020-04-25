use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Phase {
    Day(i64),
    Night(i64),
}

impl Phase {
    pub fn prev(self: &Self) -> Self {
        match self {
            Phase::Day(n) => Phase::Night(n - 1),
            Phase::Night(n) => Phase::Day(*n),
        }
    }

    pub fn next(self: &Self) -> Self {
        match self {
            Phase::Day(n) => Phase::Night(*n),
            Phase::Night(n) => Phase::Day(n + 1),
        }
    }

    pub fn num(self: &Self) -> i64 {
        match self {
            Self::Day(n) => *n,
            Self::Night(n) => *n,
        }
    }

    pub fn kind_str(self: &Self) -> &str {
        match self {
            Self::Day(_) => "day",
            Self::Night(_) => "night",
        }
    }
}
