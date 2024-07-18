use ollama_rs::Ollama;
use tokio::sync::{Mutex, OnceCell};
use url::ParseError;

use crate::configuration::get_configuration;

static OLLAMA: OnceCell<Mutex<Ollama>> = OnceCell::const_new();
static MODEL: OnceCell<String> = OnceCell::const_new();

pub async fn initialize_ai() -> Result<Ollama, ParseError> {
    let url = get_configuration()
        .expect("Failed to read configuration")
        .ollama
        .ollama_url();
    Ollama::try_new_with_history(url, 30)
}

pub async fn get_ai(
) -> Result<(tokio::sync::MutexGuard<'static, Ollama>, &'static String), ParseError> {
    OLLAMA
        .get_or_init(|| async {
            let ollama = initialize_ai().await.expect("Failed to initialize AI");
            Mutex::new(ollama)
        })
        .await;

    let model_text = MODEL
        .get_or_init(|| async {
            get_configuration()
                .expect("Failed to read configuration")
                .chat_model
        })
        .await;

    let ollama_guard = OLLAMA.get().unwrap().lock().await;
    Ok((ollama_guard, model_text))
}
