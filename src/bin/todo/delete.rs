use clap::Parser;
use inquire::Select;
use mindmap::Task;

#[derive(Parser)]
pub struct Args {}

pub fn command(_args: &Args) {
    let tasks = [
        Task {
            description: String::from("learn rust"),
            difficulty: None,
            priority: None,
            deadline: None,
        },
        Task {
            description: String::from("build mindmap cli"),
            difficulty: None,
            priority: None,
            deadline: None,
        },
        Task {
            description: String::from("build mindmap gui"),
            difficulty: None,
            priority: None,
            deadline: None,
        },
    ];
    let task_description = Select::new(
        "Select the task to delete:",
        tasks.iter().map(|task| &task.description).collect(),
    )
    .prompt()
    .expect("An error occurred!");

    println!("Task \"{}\" deleted successfully!", task_description);
}
