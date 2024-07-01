use std::error::Error as StdError;
use std::fmt;

use bytes::BytesMut;
use chrono::{NaiveDate, NaiveDateTime};
use clap::ValueEnum;
use tokio_postgres::types::{IsNull, ToSql, Type};

pub mod db;

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

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub difficulty: Option<Difficulty>,
    pub priority: Option<Priority>,
    pub deadline: Option<NaiveDate>,
}
