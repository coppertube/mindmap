use chrono::{Local, Weekday};
use clap::Parser;
use inquire::{DateSelect, Select};
use mindmap::{Difficulty, Priority, Task};

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let task = Task {
        description: inquire::prompt_text("Description:").expect("An error occurred!"),
        difficulty: Select::new(
            "Difficulty:",
            vec![Difficulty::Low, Difficulty::Medium, Difficulty::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        priority: Select::new(
            "Priority:",
            vec![Priority::Low, Priority::Medium, Priority::High],
        )
        .prompt_skippable()
        .expect("An error occurred!"),
        deadline: DateSelect::new("Deadline:")
            .with_min_date(Local::now().date_naive())
            .with_week_start(Weekday::Mon)
            .prompt_skippable()
            .expect("An error occurred!"),
    };

    task.save_to_db().await.expect("Failed to insert task");
}
