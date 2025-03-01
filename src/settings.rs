use config::{Config, Environment, File};
use serde_derive::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{env, fmt};

#[derive(Debug)]
struct ConfigError(String);
impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ConfigError {}
impl ConfigError {
    fn new(msg: String) -> Self {
        Self(msg)
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db: DBSettings,
}

#[derive(Debug, Deserialize)]
pub struct DBSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

pub fn init() -> Result<Settings, Box<dyn Error>> {
    let mut config_builder = Config::builder();
    config_builder = config_builder
        .add_source(File::new("config/default", config::FileFormat::Toml).required(false));
    config_builder = config_builder.add_source(Environment::with_prefix("APP").separator("_"));
    let string = env::var("PROFILE").unwrap_or("local".into());
    config_builder = config_builder.add_source(
        File::new(
            format!("config/{}", string).as_str(),
            config::FileFormat::Toml,
        )
        .required(false),
    );
    match config_builder.build()?.try_deserialize() {
        Ok(settings) => Ok(settings),
        Err(e) => Err(Box::new(ConfigError::new(format!(
            "Loading config error {}",
            e
        )))),
    }
}
