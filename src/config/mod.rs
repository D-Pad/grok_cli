use std::fs;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Service {
    Grok,
    Ollama
}


#[derive(Deserialize)]
pub struct Globals {
    pub terminal: bool,
    pub service: Service,
    pub stream: bool,
    pub temperature: f32,
    pub reasoning_effort: String,
    pub speak_color: String,
}


#[derive(Deserialize)]
pub struct GrokConfig {
    pub url: String, 
    pub model: String,
    pub api_key: String, 
}


#[derive(Deserialize)]
pub struct OllamaConfig {
    pub url: String, 
    pub model: String,
}


#[derive(Deserialize)]
pub struct Config {
    pub title: Option<String>,
    pub globals: Globals,
    pub grok: GrokConfig,
    pub ollama: OllamaConfig,
}


impl Config {

    pub fn active_bot(&self) -> ActiveBot<'_> {
        match self.globals.service {
            Service::Grok => ActiveBot::Grok(&self.grok),
            Service::Ollama => ActiveBot::Ollama(&self.ollama)
        }
    }

    pub fn speak_color(&self) -> &str {
        &self.globals.speak_color
    }

    pub fn api_key(&self) -> Option<&str> {
        match self.globals.service {
            Service::Grok => Some(&self.grok.api_key),
            _ => None 
        }
    }
}


pub enum ActiveBot<'a> {
    Grok(&'a GrokConfig),
    Ollama(&'a OllamaConfig)
}


impl<'a> ActiveBot<'a> {
    pub fn model(&self) -> &'a str {
        match self {
            ActiveBot::Grok(bot) => &bot.model,
            ActiveBot::Ollama(bot) => &bot.model 
        }
    }

    pub fn url(&self) -> &'a str {
        match self {
            ActiveBot::Grok(bot) => &bot.url,
            ActiveBot::Ollama(bot) => &bot.url 
        }
    }
}


pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config/config.toml")?;
    let config_data: Config = toml::from_str(&content)?;
    Ok(config_data)
}


