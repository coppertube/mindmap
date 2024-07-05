use std::error::Error;
use std::fmt;
use std::str::FromStr;

use bytes::BytesMut;
use chrono::{Local, NaiveDate};
use clap::ValueEnum;
use db::get_client;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

pub mod db;

pub mod configuration;

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

impl ToSql for Difficulty {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
        match self {
            Difficulty::Low => out.extend_from_slice(b"low"),
            Difficulty::Medium => out.extend_from_slice(b"medium"),
            Difficulty::High => out.extend_from_slice(b"high"),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "difficulty"
    }

    tokio_postgres::types::to_sql_checked!();
}

impl<'a> FromSql<'a> for Difficulty {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let s = std::str::from_utf8(raw)?;
        s.parse().map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                as Box<dyn Error + Send + Sync>
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "difficulty"
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
    ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
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
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let s = std::str::from_utf8(raw)?;
        s.parse().map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                as Box<dyn Error + Send + Sync>
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "priority"
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub difficulty: Option<Difficulty>,
    pub priority: Option<Priority>,
    pub deadline: Option<NaiveDate>,
}

impl Task {
    pub async fn insert_todo(&self) -> Result<(), Box<dyn Error>> {
        let client = get_client().await?;

        client
            .execute(
                "INSERT INTO todo (description, priority, difficulty, deadline) VALUES ($1, $2, $3, $4)",
                &[&self.description, &self.priority, &self.difficulty, &self.deadline],
            )
            .await
            .expect("Failed to insert task");

        println!("Task \"{}\" created successfully!", self.description);

        Ok(())
    }

    pub async fn delete_task(&self) -> Result<(), Box<dyn Error>> {
        let client = get_client().await?;

        client
            .execute(
                "DELETE FROM todo WHERE description = $1",
                &[&self.description],
            )
            .await
            .expect("Failed to delete task");

        println!("Task \"{}\" deleted successfully!", self.description);

        Ok(())
    }

    pub async fn list_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
        let client = get_client().await?;

        let today = Local::now().date_naive();
        let rows = client
            .query(
                "SELECT description, priority, difficulty, deadline FROM todo WHERE deadline = $1::date",
                &[&today],
            )
            .await
            .expect("Failed to fetch all tasks.");

        let tasks: Vec<Task> = rows
            .iter()
            .map(|row| Task {
                description: row.get(0),
                priority: row.get(1),
                difficulty: row.get(2),
                deadline: row.get(3),
            })
            .collect();

        Ok(tasks)
    }

    pub async fn update_task(&self, old_description: String) -> Result<(), Box<dyn Error>> {
        let client = get_client().await?;

        client
            .execute(
                "UPDATE todo SET description = $1, priority = $2, difficulty = $3, deadline = $4 WHERE description = $5",
                &[&self.description, &self.priority, &self.difficulty, &self.deadline, &old_description],
            )
            .await
            .expect("Failed to update task");

        println!("Task updated successfully!");

        Ok(())
    }
}
