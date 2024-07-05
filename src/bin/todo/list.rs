use clap::Parser;
use mindmap::Task;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let rows = Task::list_tasks()
        .await
        .expect("Failed to fetch all tasks.");

    for row in rows {
        println!("------------------------\nTask: {}", row.description);
        println!("Priority: {:?}", row.priority);
        println!("Difficulty: {:?}", row.difficulty);
        println!("Deadline: {:?}\n------------------------\n", row.deadline);
    }
}
