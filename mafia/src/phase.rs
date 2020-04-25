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

    pub fn seq(self: &Self) -> i64 {
        match self {
            Self::Day(n) => 2 * n - 1,
            Self::Night(n) => 2 * n,
        }
    }

    pub fn id(self: &Self) -> String {
        match self {
            Self::Day(n) => format!("day{}", n),
            Self::Night(n) => format!("night{}", n),
        }
    }
}
