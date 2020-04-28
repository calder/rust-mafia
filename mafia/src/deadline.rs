use serde::{Deserialize, Serialize};

use crate::phase::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Deadline {
    Never,
    Days(i64),
    Nights(i64),
}

impl Deadline {
    pub fn advance(self: &Self, phase: &Phase) -> Option<Deadline> {
        match (self.clone(), phase) {
            (Self::Days(n), Phase::Day(_)) => {
                if n <= 1 {
                    None
                } else {
                    Some(Self::Days(n - 1))
                }
            }
            (Self::Nights(n), Phase::Night(_)) => {
                if n <= 1 {
                    None
                } else {
                    Some(Self::Nights(n - 1))
                }
            }
            _ => Some(self.clone()),
        }
    }
}

impl Default for Deadline {
    fn default() -> Self {
        Deadline::Never
    }
}
