use std::error::Error;

use async_trait::async_trait;
use clap::Parser;
use ollama_rs::generation::functions::tools::Tool;
use serde_json::Value;

#[derive(Parser)]
pub struct Args {}

pub fn command(args: &Args) {
    println!("{}", args.list());
}

impl Args {
    pub fn new() -> Self {
        Args {}
    }
    pub fn list(&self) -> String {
        String::from("Tasks due today:")
    }
}
impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for Args {
    fn name(&self) -> String {
        "List Tasks".to_string()
    }

    fn description(&self) -> String {
        "Displays all the tasks pending for the current user.".to_string()
    }

    async fn run(&self, _input: Value) -> Result<String, Box<dyn Error>> {
        Ok(self.list())
    }
}
