use chrono::NaiveDateTime;
use clap::Parser;
use mindmap::Priority;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    description: String,
    #[clap(long)]
    new_description: String,
    #[clap(long)]
    new_priority: Priority,
    #[clap(long, value_parser=mindmap::parse_datetime)]
    new_deadline: NaiveDateTime,
}

pub async fn command(_args: &Args) {
    
    let client = mindmap::db::get_client().await.expect("Failed to fetch client");

    let rows = client.query("SELECT description, priority, deadline FROM todo WHERE description = $1", &[&_args.description]).await.expect("Failed to fetch task");

    if rows.is_empty() {
        println!("Task not found!");
        return;
    }

    let description = _args.description.clone();
    let new_description = _args.new_description.clone();
    let new_priority = _args.new_priority.clone();
    let new_deadline = _args.new_deadline;

    client.execute(
        "UPDATE todo SET description = $1, priority = $2, deadline = $3 WHERE description = $4",
        &[&new_description, &new_priority, &new_deadline, &description],
    ).await.expect("Failed to update task");

    println!("Task updated successfully!");
}
