use chrono::{Local, Weekday};
use clap::Parser;
use mindmap::Task;
use mindmap::{Difficulty, Priority};

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let tasks = Task::list_tasks(None)
        .await
        .expect("Internal Server Error. Try Again!");

    if tasks.is_empty() {
        println!("Task not found!");
        return;
    };

    let old_task = inquire::Select::new("Select the task to update:", tasks)
        .prompt()
        .expect("An error occurred!");

    let new_task = Task {
        description: inquire::prompt_text("New Description:").expect("An error occurred!"),
        difficulty: inquire::Select::new(
            "New Difficulty:",
            vec![Difficulty::Low, Difficulty::Medium, Difficulty::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        priority: inquire::Select::new(
            "New Priority:",
            vec![Priority::Low, Priority::Medium, Priority::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        deadline: inquire::DateSelect::new("New Deadline:")
            .with_min_date(Local::now().date_naive())
            .with_week_start(Weekday::Mon)
            .prompt_skippable()
            .expect("An error occurred!"),
    };

    new_task
        .update_task(old_task.description)
        .await
        .expect("Failed to update task");
}
