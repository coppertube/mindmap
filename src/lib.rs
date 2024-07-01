use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

use bytes::BytesMut;
use chrono::{NaiveDate, NaiveDateTime};
use clap::ValueEnum;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

pub mod db;

#[derive(Debug, Clone, ValueEnum)]
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

#[derive(Debug, Clone, ValueEnum)]
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

impl ToSql for Priority {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn StdError + Send + Sync>> {
        match self {
            Priority::Low => out.extend_from_slice(b"low"),
            Priority::Medium => out.extend_from_slice(b"medium"),
            Priority::High => out.extend_from_slice(b"high"),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "priority"
    }

    tokio_postgres::types::to_sql_checked!();
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

impl<'a> FromSql<'a> for Priority {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let s = std::str::from_utf8(raw)?;
        s.parse().map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                as Box<dyn StdError + Send + Sync>
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "priority"
    }
}

pub fn parse_datetime(s: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
}

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub difficulty: Option<Difficulty>,
    pub priority: Option<Priority>,
    pub deadline: Option<NaiveDate>,
}
