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
    // pub title: Option<String>,
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


pub fn get_config(file_path: Option<&str>) -> 
    Result<Config, Box<dyn std::error::Error>> 
{
    let default_path: &str = "config/config.toml";
    let dir_path: &str = match file_path {
        Some(p) => p,
        None => default_path
    };
    let content = fs::read_to_string(dir_path)?;
    let config_data: Config = toml::from_str(&content)?;
    Ok(config_data)
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    pub fn test_settings() -> Result<(), Box<dyn std::error::Error>> {

        let conf = get_config(Some("src/config/config.toml"))?;
     
        /*
           terminal = true  # Set this to false if running as graphical app
           stream = false
           temperature = 0.7
           reasoning_effort = "high"
           speak_color = "cyan"
           service = "ollama"
        */

        println!("\x1b[33m----------------------------------\x1b[36m");
        println!("Terminal         : {}", conf.globals.terminal);
        println!("Stream           : {}", conf.globals.stream);
        println!("Temperature      : {}", conf.globals.temperature);
        println!("Reasoning Effort : {}", conf.globals.reasoning_effort);
        println!("Speak Color      : {}\n", conf.globals.speak_color);
        println!("Model            : {}", conf.active_bot().model());
        println!("URL              : {}", conf.active_bot().url());
        println!("\x1b[33m----------------------------------\x1b[0m");
        
        Ok(())

    }

}

