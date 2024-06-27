use tokio_postgres::{NoTls, Error};
use dotenv::dotenv;
use std::env;

pub async fn insert_user(name: &str, age: i32) -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(
        "INSERT INTO users (name, age) VALUES ($1, $2)",
        &[&name, &age],
    ).await?;

    Ok(())
}

pub async fn select_all_users() -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT id, name, age FROM users", &[]).await?;
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let age: i32 = row.get(2);

        println!("Found user: {} - {} ({})", id, name, age);
    }

    Ok(())
}

pub async fn run_db_operations() -> Result<(), Error> {
    insert_user("Bhavay", 18).await?;
    select_all_users().await?;

    Ok(())
}