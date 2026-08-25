use std::str;

// Third party crates for parsing JSON and making requests
use reqwest::{blocking::{Client}, Error};
use serde::{Serialize, Deserialize};
use serde_json;

// Local crates
use crate::config::{Config, ActiveBot};
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
mod tests {
    use super::*;

    #[test]
    pub fn test_response() -> Result<(), Error> {

        // Create the client
        let mut history: Vec<Message> = Vec::new();

        // Set the url 
        // let mut url: String = String::from("http://localhost:3000/dev/chat");

        // Convert the prompt from a buffer into a string slice 
        let prompt: String = String::from("Hello, I am the user"); 
        history.push(
            Message {
                role: "user",
                content: prompt 
            }
        );
        
        // Make the request
        let resp: Result<&str, ()> = Ok(r#"{
    "id":"91d1868f-3dcf-4b92-88c8-dac32e2ce8b4",
    "object":"chat.completion",
    "created":1750535675,
    "model":"grok-3",
    "choices":[
        {
            "index":0,
            "message":
            {
                "role":"assistant",
                "content":"  # Test Response for Chat Bot Testing\n\nHello! This is a generic markdown response to assist with your chat bot testing. It's designed to include various elements for character parsing. Below, you'll find headers, code blocks, bold, and italic text.\n\n## Key Elements Included\n\nThis section demonstrates **bold text** and *italic text*. For example, you can have a combination like **bold with *italic inside*** to test nested formatting.\n\n### Subheader for Variety\n\nHere's a smaller header. It's useful for structuring content in markdown.\n\nNow, let's include a small code block and an `inline_code_snip` to simulate code parsing:\n```# This is a simple Python code snippet\nprint('Hello, this is a test message!')```\n\nFeel free to use this response multiple times for your tests. If you need adjustments, just let me know!\n\n",
                "refusal":null
            },
            "finish_reason":"stop"
        }
    ],
    "usage":{
        "prompt_tokens":36,
        "completion_tokens":3,
        "total_tokens":39,
        "prompt_tokens_details": {
            "text_tokens":36,
            "audio_tokens":0,
            "image_tokens":0,
            "cached_tokens":4
        },
        "completion_tokens_details": {
            "reasoning_tokens":0,
            "audio_tokens":0,
            "accepted_prediction_tokens":0,
            "rejected_prediction_tokens":0
        },
        "num_sources_used":0
    },
    "system_fingerprint":"fp_be0739e203"
}"#); 

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
