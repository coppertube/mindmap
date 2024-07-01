pub mod db;

use chrono::NaiveDateTime;
use clap::ValueEnum;
use tokio_postgres::types::{IsNull, ToSql, Type};
use std::error::Error as StdError;
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

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub priority: Priority,
    pub deadline: NaiveDateTime,
}
