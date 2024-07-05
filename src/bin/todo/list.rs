use chrono::Local;
use clap::Parser;
use mindmap::Task;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let today = Local::now().date_naive();
    let rows = Task::list_tasks(Some(today))
        .await
        .expect("Failed to fetch today's tasks.");

    for row in rows {
        println!("------------------------\nTask: {}", row.description);
        println!("Priority: {:?}", row.priority.unwrap());
        println!("Difficulty: {:?}", row.difficulty.unwrap());
        println!("Deadline: {:?}\n------------------------\n", row.deadline.unwrap());
    }
}
