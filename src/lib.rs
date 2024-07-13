use std::fmt;
use std::str::FromStr;

use chrono::NaiveDate;

pub mod configuration;
pub mod model;

pub enum Difficulty {
    Low,
    Medium,
    High,
}
impl FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Difficulty::Low),
            "medium" => Ok(Difficulty::Medium),
            "high" => Ok(Difficulty::High),
            _ => Err(format!("Invalid difficulty: {}", s)),
        }
    }
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

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Task: {}", self.description)?;

        match &self.difficulty {
            Some(difficulty) => writeln!(f, "Difficulty: {}", difficulty)?,
            None => writeln!(f, "Difficulty: None")?,
        };
        match &self.priority {
            Some(priority) => writeln!(f, "Priority: {}", priority)?,
            None => writeln!(f, "Priority: None")?,
        };

        match &self.deadline {
            Some(deadline) => writeln!(f, "Deadline: {}", deadline)?,
            None => writeln!(f, "Deadline: None")?,
        };

        Ok(())
    }
}
