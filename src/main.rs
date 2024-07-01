mod llm;
use langchain_rust::schemas::Document;
use llm::{clean_add_to_db, recommendation};

#[tokio::main]
async fn main() {
    let store = llm::initialize_store().await;
    let model = llm::initialize_ai();

    let tasks: Vec<&str> = vec![
        "Buy groceries",
        "Clean the house",
        "Pay bills",
        "Attend meeting",
        "Exercise",
        "Read a book",
        "Cook dinner",
        "Call a friend",
        "Water the plants",
        "Walk the dog",
    ];
    let task_documents: Vec<Document> = tasks
        .iter()
        .map(|text| Document::new(text.to_string()))
        .collect();
    llm::store_documents(&store, task_documents).await;

    clean_add_to_db(&store, &model).await;

    recommendation(&store, model).await;
}
