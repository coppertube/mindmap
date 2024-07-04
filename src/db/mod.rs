use std::env;

use chrono::{Local, NaiveDate};
use dotenv::dotenv;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, Error, NoTls, Row};

use crate::{Difficulty, Priority, Task};

static CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn init_client() -> Result<Client, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn get_client() -> Result<&'static Client, Error> {
    CLIENT.get_or_try_init(init_client).await
}

pub async fn get_all_tasks() -> Result<Vec<Task>, Error> {
    let client = get_client().await?;

    let rows = client
        .query(
            "SELECT description, priority, difficulty, deadline FROM todo",
            &[],
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

pub async fn insert_todo(
    description: String,
    priority: Option<Priority>,
    difficulty: Option<Difficulty>,
    deadline: Option<NaiveDate>,
) -> Result<(), Error> {
    let client = get_client().await?;

    client
        .execute(
            "INSERT INTO todo (description, priority, difficulty, deadline) VALUES ($1, $2, $3, $4)",
            &[&description, &priority, &difficulty, &deadline],
        )
        .await
        .expect("Failed to insert task");

    println!("Task \"{}\" created successfully!", description);

    Ok(())
}

pub async fn delete_task(description: String) -> Result<(), Error> {
    let client = get_client().await?;

    client
        .execute("DELETE FROM todo WHERE description = $1", &[&description])
        .await
        .expect("Failed to delete task");

    println!("Task \"{}\" deleted successfully!", description);

    Ok(())
}

pub async fn list_tasks() -> Result<Vec<Row>, Error> {
    let client = get_client().await?;

    let today = Local::now().date_naive();
    let rows = client
        .query(
            "SELECT description, priority, difficulty, deadline FROM todo WHERE deadline = $1::date",
            &[&today],
        )
        .await
        .expect("Failed to fetch tasks");

    Ok(rows)
}

pub async fn update_task(
    new_description: String,
    new_priority: Option<Priority>,
    new_difficulty: Option<Difficulty>,
    new_deadline: Option<NaiveDate>,
    old_description: String,
) -> Result<(), Error> {
    let client = get_client().await?;
    client
        .execute(
            "UPDATE todo SET description = $1, priority = $2, difficulty = $3, deadline = $4 WHERE description = $5",
            &[&new_description, &new_priority, &new_difficulty, &new_deadline, &old_description],
        )
        .await
        .expect("Failed to update task");

    println!("Task updated successfully!");

    Ok(())
}
