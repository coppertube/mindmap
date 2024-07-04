use clap::Parser;
use inquire::Select;
use mindmap::db::{delete_task, get_all_tasks};

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

    delete_task(task_description.to_string())
        .await
        .expect("Failed to delete task.");
}
