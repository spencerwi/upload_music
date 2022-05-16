use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct OutputConfig {
    pub upload_dir: String,
    pub filename_pattern: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub interface: Option<String>,
    pub output: OutputConfig,
}

pub fn load_config() -> Config {
    match fs::read_to_string("upload_music.toml") {
        Ok(config_file_contents) => {
            match toml::from_str(&config_file_contents) {
                Ok(config_struct) => {
                    return config_struct;
                },
                Err(e) => { panic!("Error parsing config file: {}", e); }
            }
        },
        Err(e) => { panic!("Error reading config file: {}", e); }
    }
}
