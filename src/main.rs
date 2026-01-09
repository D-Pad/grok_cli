use std::{fs::create_dir, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
mod config;
mod chat;


fn main() {
    // Get config file for application 
    let app_config =  match config::get_config() {
        Ok(data) => data,
        Err(_) => panic!("Could not read config file") 
    };

    // Create the directory for storing history, if it doesn't exist already
    let path_buf = PathBuf::from("chat/cache/history");
    if !path_buf.is_dir() {
        let _ = create_dir(path_buf.as_path()); 
    }

    // Set the session history file name
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_secs();
    let history_file_name = format!("chat/cache/history/grok_{ts}.json");

    // Run the app 
    match app_config.globals.terminal {
        true => chat::terminal_app(&app_config, history_file_name),
        false => chat::http_app()
    }
}

