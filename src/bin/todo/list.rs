use chrono::Local;
use clap::Parser;
use mindmap::db::get_client;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let client = get_client().await.expect("Failed to fetch client");
    let today = Local::now().date_naive();
    let rows = client
        .query(
            "SELECT description, priority, difficulty, deadline FROM todo WHERE deadline = $1::date",
            &[&today],
        )
        .await
        .expect("Failed to fetch tasks");

    for row in rows {
        let description: String = row.get(0);
        let priority: mindmap::Priority = row.get(1);
        let difficulty: mindmap::Difficulty = row.get(2);
        let deadline: chrono::NaiveDate = row.get(3);

        println!("------------------------\nTask: {}", description);
        println!("Priority: {:?}", priority);
        println!("Difficulty: {:?}", difficulty);
        println!("Deadline: {}\n------------------------\n", deadline);
    }
}
