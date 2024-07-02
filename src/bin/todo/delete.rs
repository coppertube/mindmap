use clap::Parser;
use inquire::Select;
use mindmap::db::{get_all_tasks, get_client};

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let tasks = get_all_tasks()
        .await
        .expect("Internal Server Error. Try Again!");
    let task_description = Select::new(
        "Select the task to delete:",
        tasks.iter().map(|task| &task.description).collect(),
    )
    .prompt()
    .expect("An error occurred!");

    let client = get_client().await.expect("Failed to fetch client");
    client
        .execute(
            "DELETE FROM todo WHERE description = $1",
            &[&task_description],
        )
        .await
        .expect("Failed to delete task");

    println!("Task \"{}\" deleted successfully!", task_description);
}
