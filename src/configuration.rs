use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub debug: bool,
    pub chat_model: String,
    pub ollama: OllamaConfiguration,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub username: String,
}

#[derive(Deserialize)]
pub struct OllamaConfiguration {
    ollama_scheme: String,
    ollama_host: String,
    ollama_port: u16,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

impl OllamaConfiguration {
    pub fn ollama_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.ollama_scheme, self.ollama_host, self.ollama_port
        )
    }
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let mut builder = Config::builder().add_source(File::with_name("config"));
    if let Some(mut path) = dirs::config_dir() {
        path.push("mindmap");
        path.push("config.toml");
        if path.exists() {
            builder = builder.add_source(File::from(path));
        }
    }
    builder = builder.add_source(Environment::with_prefix("MINDMAP").separator("_"));

    let configuration = builder.build()?;
    configuration.try_deserialize::<Configuration>()
}
