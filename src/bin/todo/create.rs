use clap::Parser;
use mindmap::Task;

#[derive(Parser)]
pub struct Args {}

pub fn command(_args: &Args) {
    let task = Task {
        description: String::from("don't slack off"),
    };
    println!("Task \"{}\" created successfully!", task.description);
}
