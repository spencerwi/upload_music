use serde::Deserialize;
use std::fs;
use crate::errors::AppError;

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
    match fs::read_to_string("upload_music.toml") {
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

