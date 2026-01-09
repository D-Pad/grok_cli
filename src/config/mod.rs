use std::fs;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct Config {
    pub globals: Globals, 
    pub auth: Authentication, 
    pub bot: Bot,
}


#[derive(Deserialize)]
pub struct Globals {
    pub terminal: bool 
}


#[derive(Deserialize)]
pub struct Authentication {
    pub api_key: String
}


#[derive(Deserialize)]
pub struct Bot {
    pub url: String, 
    pub model: String,
    pub stream: bool,
    pub temperature: f32,
    pub reasoning_effort: String,
    pub speak_color: String,
}


pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config/config.toml")?;
    let config_data: Config = toml::from_str(&content)?;
    Ok(config_data)
}


