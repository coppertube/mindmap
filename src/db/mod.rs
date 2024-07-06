use tokio::sync::OnceCell;
use tokio_postgres::{Client, Error, NoTls};

use crate::configuration::get_configuration;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn init_client() -> Result<Client, Error> {
    let database_url = get_configuration()
        .expect("Failed to read configuration")
        .database
        .connection_string();

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
