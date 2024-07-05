use clap::Parser;
use inquire::Select;
use mindmap::Task;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let tasks = Task::list_tasks(false)
        .await
        .expect("Internal Server Error. Try Again!");

    let task_description = Select::new(
        "Select the task to delete:",
        tasks.iter().map(|task| &task.description).collect(),
    )
    .prompt()
    .expect("An error occurred!");

    let task = tasks
        .iter()
        .find(|task| task.description == *task_description)
        .expect("Task not found!");

    task.delete_from_db().await.expect("Failed to delete task");
}
