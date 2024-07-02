use crate::Task;
use dotenv::dotenv;
use std::env;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, Error, NoTls};

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
