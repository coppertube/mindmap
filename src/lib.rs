pub mod db;

use chrono::NaiveDateTime;
use clap::ValueEnum;
use tokio_postgres::types::{IsNull, ToSql, FromSql, Type};
use std::error::Error as StdError;
use std::str::FromStr;
use bytes::BytesMut;

#[derive(Debug, Clone, ValueEnum)]
pub enum Priority {
    Low,
    Neutral,
    Unknown,
    High,
    Critical,
}

impl ToSql for Priority {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn StdError + Send + Sync>> {
        match self {
            Priority::Low => out.extend_from_slice(b"low"),
            Priority::Neutral => out.extend_from_slice(b"neutral"),
            Priority::Unknown => out.extend_from_slice(b"unknown"),
            Priority::High => out.extend_from_slice(b"high"),
            Priority::Critical => out.extend_from_slice(b"critical"),
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
            "neutral" => Ok(Priority::Neutral),
            "unknown" => Ok(Priority::Unknown),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

impl<'a> FromSql<'a> for Priority {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let s = std::str::from_utf8(raw)?;
        s.parse().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)) as Box<dyn StdError + Send + Sync>)
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
    pub priority: Priority,
    pub deadline: NaiveDateTime,
}
