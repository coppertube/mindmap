use std::env;
use std::io::Write;

use dotenv::dotenv;
use futures::StreamExt;
use langchain_rust::chain::{Chain, StuffDocumentBuilder};
use langchain_rust::embedding::ollama::ollama_embedder::OllamaEmbedder;
use langchain_rust::language_models::llm::LLM;
use langchain_rust::llm::ollama::client::Ollama;
use langchain_rust::schemas::Document;
use langchain_rust::vectorstore::pgvector::{Store, StoreBuilder};
use langchain_rust::vectorstore::VectorStore;
use langchain_rust::{add_documents, prompt_args, similarity_search};
use tokio::io::{self, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn initialize_store() -> Store {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let embedder_model = env::var("EMBEDDER_MODEL").expect("EMBEDDER_MODEL must be set");
    let embedder = OllamaEmbedder::default().with_model(embedder_model);

    StoreBuilder::new()
        .embedder(embedder)
        .pre_delete_collection(true)
        .connection_url(&database_url)
        .vector_dimensions(1024)
        .build()
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error initializing store: {}", e);
            std::process::exit(1);
        })
}

pub fn initialize_ai() -> Ollama {
    dotenv().ok();
    let llm_name = env::var("LLM_NAME").expect("LLM_NAME must be set");
    Ollama::default().with_model(llm_name)
}

pub async fn store_documents(store: &Store, documents: Vec<Document>) {
    match add_documents!(store, &documents).await {
        Ok(_) => println!("Documents added successfully!"),
        Err(e) => {
            eprintln!("Error searching documents: {}", e);
        }
    }
}

async fn similarity_search(store: &Store, search_input: &str, limit: usize) -> Vec<String> {
    match similarity_search!(store, search_input, limit).await {
        Ok(data) => data.into_iter().map(|doc| doc.page_content).collect(),
        Err(e) => {
            eprintln!("Error searching documents: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn clean_add_to_db(store: &Store, ollama: &Ollama) {
    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    print!(
        "Please enter the text you want to add/ add multiple by separating them by semicolons: "
    );
    std::io::stdout().flush().unwrap();

    reader.read_line(&mut input).await.unwrap();
    let input = ollama.invoke("Please process the input so that it is ready to be converted into vectors. Clean and normalize the text, then return the output separated by semicolons:{input}").await.unwrap();

    let list: Vec<&str> = input.split(';').collect();

    let documents: Vec<Document> = list
        .iter()
        .map(|text| Document::new(text.trim().to_string()))
        .collect();

    store_documents(store, documents).await;
    std::io::stdout().flush().unwrap();
}

pub async fn recommendation(store: &Store, ollama: Ollama) {
    let mut input = String::new();
    print!("Please enter your relevant query: ");
    std::io::stdout().flush().unwrap();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input).await.unwrap();
    let input = input.trim_end();
    let similar_documents = similarity_search(store, input, 5).await;

    let chain = StuffDocumentBuilder::new().llm(ollama).build().unwrap();
    let input = prompt_args! {
        "input_documents"=>vec![
            Document::new(format!(
                "\nQuestion: {}\nAnswer: {}\n",
                "\"{input}\" which tasks match my requirements?", similar_documents.join(";")
            )),
        ],
        "question"=>"Which of these task do you think I should do next?"
    };
    let mut stream = chain.stream(input).await.unwrap();
    let mut sdout = stdout();
    while let Some(res) = stream.next().await {
        let data = res.unwrap();
        sdout.write_all(data.content.as_bytes()).await.unwrap();
        sdout.flush().await.unwrap();
    }
    sdout.write_all(b"\n").await.unwrap();
    sdout.flush().await.unwrap();
}
