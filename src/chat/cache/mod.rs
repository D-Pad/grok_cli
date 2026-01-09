// Built in 
use std::fs::File;
use std::io::Write;


// Local imports
use super::response::Message;
use serde_json;


pub fn cache_history(
    file_name: &str, 
    history: &Vec<Message>) -> std::io::Result<()> {

    // Serialize json data
    let json_data: String = serde_json::to_string(history)
        .expect("Failed to serialize history"); 
    
    // Write the data to file
    let mut file = File::create(file_name)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

