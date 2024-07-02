use std::fmt;

use chrono::NaiveDate;

pub enum Difficulty {
    Low,
    Medium,
    High,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Difficulty::Low => write!(f, "Low"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::High => write!(f, "High"),
        }
    }
}

pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

pub struct Task {
    pub description: String,
    pub difficulty: Option<Difficulty>,
    pub priority: Option<Priority>,
    pub deadline: Option<NaiveDate>,
}
