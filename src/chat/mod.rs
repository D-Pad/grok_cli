use std::io::{self, Read, Write};


// Local modules
mod text_tools;
mod response;
mod cache;
use crate::config::Config;


const CHUNK_SIZE: usize = 16 * 1024;


// If this app is running as a terminal version
pub fn terminal_app(config: &Config, history_file_name: String) {

    // Create a buffer of a fixed size, to read and write data to.
    let mut buffer = [0u8; CHUNK_SIZE];
    
    // Lock stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Initialize chat history
    let mut history: Vec<response::Message> = Vec::new();
    history.push(
        response::Message {
            role: "system", 
            content: "You are a helpful AI assistant".to_string()
        }
    );

    loop { 
   
        handle.write_all(b"\n\x1b[31mUser:\x1b[0m\n  ").unwrap();
        handle.flush().unwrap();
        
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break
        };
         
        // Send the request, to get user input
        let _ = handle.write_all(b"\n\x1b[32mBot: "); 
        let user_prompt: &[u8] = &buffer[..num_read];
        
        let ai_response = response::get_response(
            &user_prompt, 
            &mut history,
            &config
        );
        
        let response: &str = match ai_response {
            Ok(_) => {
                match &history.get(&history.len() - 1) {
                    Some(val) => &val.content,
                    None => "Couldn't find response in 'history'"
                }
            },
            Err(_) => "Request failed"
        };
                
        // Display the response for the user
        response::display::type_message(
            response, 
            &mut handle, 
            &config.globals.speak_color
        ); 
        cache::cache_history(&history_file_name, &history).unwrap(); 
    };
}


pub fn http_app() {
    println!("Not implemented");
}

