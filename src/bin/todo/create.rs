use chrono::NaiveDateTime;
use clap::Parser;
use mindmap::{Task, Priority};
use mindmap::db::get_client;

#[derive(Parser, Clone, Debug)]
pub struct Args {
    #[clap(long)]
    description: String,
    #[clap(long)]
    priority: Priority,
    #[clap(long, value_parser=parse_datetime)]
    deadline: NaiveDateTime,
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
}

pub async fn command(args: &Args) {

    let task = Task {
        description: args.description.clone(),
        priority: args.priority.clone(),
        deadline: args.deadline,
    };

    let client = get_client().await.expect("Failed to get client");
    client.execute(
        "INSERT INTO todo (description, priority, deadline) VALUES ($1, $2, $3)",
        &[&task.description, &task.priority, &task.deadline],
    ).await.expect("Failed to insert task");

    println!("Task \"{}\" created successfully!", task.description);
}
