use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub debug: bool,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub username: String,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
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
