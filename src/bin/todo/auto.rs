use std::sync::Arc;

use clap::Parser;
use inquire::prompt_text;
use mindmap::model::get_ai;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::functions::{FunctionCallRequest, NousFunctionCall};

use crate::{create, list};

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: &Args) {
    let (mut ollama, model) = get_ai().await.unwrap();
    let create = Arc::new(create::Args::new());
    let list = Arc::new(list::Args::new());
    let parser = Arc::new(NousFunctionCall::new());

    let response = ollama
        .send_function_call_with_history(
            FunctionCallRequest::new(
                model.clone(),
                vec![create, list],
                vec![ChatMessage::user(
                    prompt_text("Prompt:").expect("An error occurred!"),
                )],
            ),
            parser,
            "default".to_string(),
        )
        .await
        .unwrap();

    let assistant_message = response.message.unwrap().content;
    println!("{}", assistant_message);
}
