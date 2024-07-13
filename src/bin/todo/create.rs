use std::error::Error;

use async_trait::async_trait;
use chrono::{Local, NaiveDate, Weekday};
use clap::Parser;
use inquire::{DateSelect, Select};
use mindmap::{Difficulty, Priority, Task};
use ollama_rs::generation::functions::tools::Tool;
use serde_json::{json, Value};

#[derive(Parser)]
pub struct Args {}

pub fn command(args: &Args) {
    println!("{}", args.create_task(None, None, None, None));
}
impl Args {
    pub fn new() -> Self {
        Args {}
    }
    pub fn create_task(
        &self,
        description_input: Option<&str>,
        difficulty_input: Option<Difficulty>,
        priority_input: Option<Priority>,
        deadline_input: Option<NaiveDate>,
    ) -> String {
        let today = Local::now().date_naive();
        let task = Task {
            description: match description_input {
                Some(description) => description.to_string(),
                None => inquire::prompt_text("Description").expect("An error occurred!"),
            },
            difficulty: difficulty_input.or_else(|| {
                Select::new(
                    "Difficulty",
                    vec![Difficulty::Low, Difficulty::Medium, Difficulty::High],
                )
                .prompt_skippable()
                .expect("An error occurred!")
            }),
            priority: priority_input.or_else(|| {
                Select::new(
                    "Priority",
                    vec![Priority::Low, Priority::Medium, Priority::High],
                )
                .prompt_skippable()
                .expect("An error occurred!")
            }),
            deadline: match deadline_input {
                Some(date) if date >= today => Some(date),
                Some(_) | None => DateSelect::new("Deadline")
                    .with_min_date(today)
                    .with_week_start(Weekday::Mon)
                    .prompt_skippable()
                    .expect("An error occurred!"),
            },
        };
        format!("Task \"{}\" created successfully!", task)
    }
}

impl Default for Args {
    fn default() -> Self {
        Args::new()
    }
}

#[async_trait]
impl Tool for Args {
    fn name(&self) -> String {
        "Create Task".to_string()
    }

    fn description(&self) -> String {
        "Create a new task for the current user.".to_string()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "The description of the task."
                },
                "difficulty": {
                    "type": "string",
                    "enum": ["low", "medium", "high"],
                    "description": "The difficulty of the task."
                },
                "priority": {
                    "type": "string",
                    "enum": ["low", "medium", "high"],
                    "description": "The priority of the task."
                },
                "deadline": {
                    "type": "string",
                    "format": "date",
                    "description": format!("Today is {},deadline of the task should be equal or greater than today.",Local::now().date_naive())
                }
            },
            "required": []
        })
    }

    async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
        let description_input = input["description"].as_str();
        let difficulty_input = input["difficulty"]
            .as_str()
            .and_then(|s| s.parse::<Difficulty>().ok());
        let priority_input = input["priority"]
            .as_str()
            .and_then(|s| s.parse::<Priority>().ok());
        let deadline_input = input["deadline"]
            .as_str()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        Ok(self.create_task(
            description_input,
            difficulty_input,
            priority_input,
            deadline_input,
        ))
    }
}
