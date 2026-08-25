use std::{str, fs};

// Third party crates for parsing JSON and making requests
use reqwest::{blocking::{Client}, Error};
use serde::{Serialize, Deserialize};
use serde_json;

// Local crates
use crate::config::{Config};
pub mod grok;
pub mod display;


// Message struct for the 'message' field in the post request.
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: &'static str,
    pub content: String
}


#[derive(Serialize)]
struct Request<'a> {
    messages: Vec<Message>,
    model: &'a str,
    stream: &'a bool,
    temperature: &'a f32,
    reasoning_effort: &'a str 
}


pub fn get_response(
    buffer: &[u8], 
    history: &mut Vec<Message>,
    config: &Config
) -> Result<(), Error> {
  
    // Create the client
    let http_client: Client = Client::new();

    // Convert the prompt from a buffer into a string slice 
    let prompt: String = str::from_utf8(&buffer).unwrap().to_string(); 
    history.push(
        Message {
            role: "user",
            content: prompt 
        }
    );

    let request = Request {
        messages: history.clone(),
        model: config.active_bot().model(),
        stream: &config.globals.stream,
        temperature: &config.globals.temperature,
        reasoning_effort: &config.globals.reasoning_effort
    };

    let json_payload = serde_json::to_string(&request).unwrap();
 
    // Make the request
    let mut query = http_client
        .post(config.active_bot().url())
        .header("Content-Type", "application/json") 
        .body(json_payload);
 
    if let Some(key) = &config.api_key() {
        query = query.header("Authorization", format!("Bearer {}", key)); 
    };

    let resp = query.send();

    // Get the text from the request
    let body: String = match resp {
        Ok(resp) => {
            resp.text().ok().unwrap()
        },
        Err(_) => String::from("Request failed") 
    };

    let ai_response: grok::Grok = serde_json::from_str(&body)
        .expect("Failed to deserialize response");

    // Append the response to the history
    history.push(
        Message {
            role: "system",
            content: match ai_response.choices.get(0) {
                Some(resp) => resp.message.content.clone(),
                None => String::from("Failed find message in response data")
            }
        }
    );

    Ok(())
}


#[cfg(test)]
mod response_tests {
    use super::*;

    #[test]
    pub fn test_response() -> Result<(), Error> {

        // Create the client
        let mut history: Vec<Message> = Vec::new();

        // Convert the prompt from a buffer into a string slice 
        let prompt: String = String::from("Hello, I am the user"); 
        history.push(
            Message {
                role: "user",
                content: prompt 
            }
        );
        
        // Make the request
        let test_resp_text = match 
            fs::read_to_string("src/chat/response/test_response.json") {
                Ok(d) => d, 
                Err(_) => panic!("Failed to read test response file")
            };
            

        let resp: Result<&str, ()> = Ok(&test_resp_text);

        // Get the text from the request
        let body: String = match resp {
            Ok(resp) => {
                resp.to_string()
            },
            Err(_) => String::from("Request failed") 
        };

        let ai_response: grok::Grok = serde_json::from_str(&body)
            .expect("Failed to deserialize response");

        // Append the response to the history
        history.push(
            Message {
                role: "system",
                content: match ai_response.choices.get(0) {
                    Some(resp) => resp.message.content.clone(),
                    None => String::from("Failed find message in response data")
                }
            }
        );

        // Create handle here, for testing the 'type_message' function
        use std::io::stdout; 
        let stdout = stdout();
        let mut handle = stdout.lock();

        let message_struct = &history.get(1);
        let message: &str = match message_struct {
            Some(m) => &m.content,
            None => panic!("Could not parse sample message")
        };

        display::type_message(&message, &mut handle, "cyan");        
        Ok(())
    }
}

