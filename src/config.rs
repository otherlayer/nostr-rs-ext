use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use tracing::warn;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct Network {
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct Plugins {
    pub folder: String,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub network: Network,
    pub plugins: Plugins,
}

impl Settings {
    #[must_use]
    pub fn new(config_file_name: &Option<String>) -> Self {
        let default_settings = Self::default();
        let from_file = Self::new_from_default(&default_settings, config_file_name);
        match from_file {
            Ok(f) => f,
            Err(e) => {
                warn!("Error reading config file ({:?})", e);
                default_settings
            }
        }
    }

    fn new_from_default(default: &Settings, config_file_name: &Option<String>) -> Result<Self, ConfigError> {
        let default_config_file_name = "config.toml".to_string();
        let config_file: String = match config_file_name {
            Some(value) => value.to_owned(),
            None => default_config_file_name
        };
        let builder = Config::builder();
        let def = Config::try_from(default)?;
        let our = File::with_name(&config_file);
        let config = builder
            .add_source(def)
            .add_source(our)
            .build()?;

        let settings: Settings = config.try_deserialize()?;

        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            network: Network {
                port: 50051,
                address: "0.0.0.0".to_owned(),
            },
            plugins: Plugins { 
                folder: "plugins".to_owned(),
                names: vec![],
            },
        }
    }
}
