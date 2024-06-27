mod db;

#[tokio::main]
async fn main() {
    if let Err(e) = db::run_db_operations().await {
        eprintln!("Error running db operations: {}", e);
    }

    println!("Hello, world!");
}
