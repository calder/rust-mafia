use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Deadline {
    Never,
    Phases(usize),
}

impl Deadline {
    pub fn next(self: &Self) -> Option<Deadline> {
        match self {
            Self::Phases(0) => None,
            Self::Phases(1) => None,
            Self::Phases(n) => Some(Self::Phases(n - 1)),
            Self::Never => Some(Self::Never),
        }
    }
}

impl Default for Deadline {
    fn default() -> Self {
        Deadline::Never
    }
}
