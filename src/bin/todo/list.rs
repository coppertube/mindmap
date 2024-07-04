use clap::Parser;
use mindmap::db::list_tasks;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let rows = list_tasks().await.expect("Failed to list tasks.");

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
