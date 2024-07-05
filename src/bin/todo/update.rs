use chrono::{Local, Weekday};
use clap::Parser;
use mindmap::db::get_all_tasks;
use mindmap::{Difficulty, Priority, Task};

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let tasks = get_all_tasks()
        .await
        .expect("Internal Server Error. Try Again!");

    if tasks.is_empty() {
        println!("Task not found!");
        return;
    };

    let task_choices: Vec<String> = tasks
        .iter()
        .map(|task| {
            format!(
                "\nDescription: {}\nDifficulty: {}\nPriority: {}\nDeadline: {}",
                task.description,
                task.difficulty
                    .as_ref()
                    .map_or("Not set".to_string(), |d| d.to_string()),
                task.priority
                    .as_ref()
                    .map_or("Not set".to_string(), |p| p.to_string()),
                task.deadline
                    .map_or("Not set".to_string(), |d| d.to_string())
            )
        })
        .collect();

    let task_description = inquire::Select::new("Select the task to update:", task_choices)
        .prompt()
        .expect("An error occurred!");

    let new_task = Task {
        description: inquire::prompt_text("New Description").expect("An error occurred!"),
        difficulty: inquire::Select::new(
            "New Difficulty",
            vec![Difficulty::Low, Difficulty::Medium, Difficulty::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        priority: inquire::Select::new(
            "New Priority",
            vec![Priority::Low, Priority::Medium, Priority::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        deadline: inquire::DateSelect::new("New Deadline")
            .with_min_date(Local::now().date_naive())
            .with_week_start(Weekday::Mon)
            .prompt_skippable()
            .expect("An error occurred!"),
    };

    new_task
        .update_task(task_description)
        .await
        .expect("Failed to update task");
}
