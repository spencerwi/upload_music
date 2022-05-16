use serde::Deserialize;
use std::fs;
use crate::errors::AppError;

use etcetera::app_strategy;
use etcetera::app_strategy::AppStrategy;
use etcetera::app_strategy::AppStrategyArgs;

#[derive(Deserialize)]
pub struct OutputConfig {
    pub upload_dir: String,
    pub filename_pattern: String,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_interface")]
    pub interface: String,
    pub output: OutputConfig,
}

pub fn load_config() -> Result<Config, AppError> {
    let config_location_strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com.spencerwi".to_string(), 
        author: "spencerwi".to_string(), 
        app_name: "upload_music".to_string()
    }).unwrap();

    let config_file_path = config_location_strategy.config_dir().join("upload_music.toml");
    if !config_file_path.exists() {
        return Err(AppError::InvalidConfig { 
            cause: format!("File does not exist at {}", config_file_path.to_path_buf().to_string_lossy()) 
        });
    }

    match fs::read_to_string(config_file_path) {
        Ok(config_file_contents) => {
            match toml::from_str(&config_file_contents) {
                Ok(config_struct) => {
                    return Ok(config_struct);
                },
                Err(e) => { 
                    return Err(AppError::InvalidConfig { 
                        cause: format!("{}", e) 
                    }); 
                }
            }
        },
        Err(e) => { 
            return Err(AppError::InvalidConfig { 
                cause: format!("{}", e)
            }); 
        }
    }
}

// defaults
fn default_port() -> u16 {
    return 5551;
}
fn default_interface() -> String {
    return "127.0.0.1".to_string();
}

