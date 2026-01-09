use std::io::{self, Read, StdoutLock, Write};
use std::{thread, time::Duration};
use std::collections::HashMap;


// Third party imports 
use termion;
use unicode_width::UnicodeWidthStr;

// Local 
use crate::chat::text_tools::color_strings::{self}; 


// Display the AI response in the terminal.
// 'type' as in using a typewriter
pub fn type_message(
    ai_response: &str, 
    handle: &mut StdoutLock, 
    default_color: &str) {

    fn preformat(
        source_string: &str, 
        screen_width: u16,
        default_color: &str) -> String {
      
        // Color keys
        let header_color_keys = HashMap::from([
            (0, default_color), 
            (1, "yellow"),
            (2, "green"),
            (3, "blue"),
            (4, "red"),
            (5, "magenta"),
            (6, "white")
        ]);

        let mut formatted: String = String::new();
        let mut word_buffer: String = String::new(); 
        let mut line_buffer: String = format!(
            "{}\n  ", 
            color_strings::ansi_code(&default_color)
        ); 
        let newline: &str = "\n  ";
        let max_screen_width: usize = screen_width as usize;
        
        // ------------ VARIABLES FOR TRACKING FORMATTING ---------- // 
        let mut last_char: char = 'A'; 

        // Headers 
        let mut header_count: u8 = 0;
        let mut parsing_header: bool = false;

        // Code blocks 
        let mut in_code_block: bool = false;
        let mut writing_code: bool = false;
        let mut consecutive_back_ticks: u8 = 0;
        let mut multi_line_code_block: bool = false;

        // Bold and italics
        let mut bold: bool = false;
        let mut italic: bool = false;
        let mut asteric_count: u8 = 0;

        for c in source_string.chars() {
            
            // Check format characters before whitespace and plain text chars.
            
            // White space, plain text, and newline chars
            if c == ' ' {
               
                // Increment the word count
                let line_width: usize = UnicodeWidthStr::width(
                    line_buffer.as_str()
                );
                
                // Get the length of the word buffer
                word_buffer.push_str(&c.to_string()); 
                let word_width: usize = UnicodeWidthStr::width(
                    word_buffer.as_str()
                );
                
                // If the word count is as wide, or wider than the screen,
                // then push a newline sequence onto the buffer.
                if line_width + word_width >= max_screen_width - 1 {
                    line_buffer.push_str(newline);
                    formatted.push_str(&line_buffer);
                    line_buffer = String::new();
                }

                // Now push the word onto the line, and start a new word, but
                // filter out double spaces
                if &word_buffer != " " { 
                    line_buffer.push_str(&word_buffer);
                }; 
                word_buffer = String::new(); 
            
            } 

            else if c == '\n' {               
                
                // Increment the word count
                let line_width: usize = UnicodeWidthStr::width(
                    line_buffer.as_str()
                );
                let word_width: usize = UnicodeWidthStr::width(
                    word_buffer.as_str()
                );
               
                if line_width + word_width >= max_screen_width - 1 {
                    line_buffer.push_str(newline);
                };
                
                line_buffer.push_str(&word_buffer);
                formatted.push_str(&line_buffer);
                formatted.push_str(newline);
           
                line_buffer = String::new();
                word_buffer = String::new();
                
                // ----------------- RESET ON NEWLINE -------------------- //
                // Some variables need to be reset when a newline character
                // is detected. Such as header colors
                if parsing_header || !writing_code {
                    line_buffer.push_str(
                        color_strings::ansi_code("")
                    );
                    parsing_header = false;
                }
            } 

            else {
                
                // Header counting
                if c == '#' && !in_code_block {
                    header_count += 1;
                    parsing_header = true; 
                    continue;
                }

                if parsing_header && header_count > 0 {
                    line_buffer.push_str(
                        &format!(
                            "{}{}", 
                            color_strings::ansi_code("bold"),
                            color_strings::ansi_code(
                                header_color_keys[&header_count]
                            ),
                        )
                    );
                    header_count = 0;
                }

                // Code block parsing
                if c == '`' && !in_code_block && !writing_code {
                    in_code_block = true; 
                    word_buffer.push_str(
                        &format!(
                            "{}{}",
                            color_strings::ansi_code("green"),
                            color_strings::ansi_code("!black")
                        ) 
                    );               
                
                } else if c == '`' && last_char == '`' {
                    consecutive_back_ticks += 1;
                
                } else if in_code_block && !writing_code{
                    writing_code = true;
                
                } else if c == '`' && writing_code {
                    if !multi_line_code_block { 
                        in_code_block = false; 
                        writing_code = false;
                        word_buffer.push_str(color_strings::ansi_code("")); 
                    }
                    else {
                        word_buffer.push_str(newline);

                    }
                };

                // Bold / Italic parsing 
                if c == '*' {
                    asteric_count += 1;

                    if asteric_count == 3 && (bold || italic) {
                        line_buffer.push_str(color_strings::ansi_code(""));
                    }

                    if !in_code_block {
                        continue;
                    }
                
                } else {
                    
                    // Check for bold or italic entry signals
                    if asteric_count == 1 && !italic {
                        italic = true;
                        line_buffer.push_str(
                            color_strings::ansi_code("italic")
                        ); 
                    
                    } else if asteric_count == 2 && !bold {
                        bold = true; 
                        line_buffer.push_str(
                            color_strings::ansi_code("bold")
                        ); 
                    
                    // Check for exit signals.
                    } else if asteric_count == 1 && italic {
                        italic = false;
                        line_buffer.push_str(color_strings::ansi_code(""));
                        if bold {
                            line_buffer.push_str(
                                color_strings::ansi_code("bold")
                            );
                        };
                    
                    } else if asteric_count == 2 && bold {
                        bold = false;
                        line_buffer.push_str(color_strings::ansi_code(""));
                        if italic {
                            line_buffer.push_str(
                                color_strings::ansi_code("italic")
                            );
                        };
                    };
                    asteric_count = 0;
                };


                if c != '`' {
                    word_buffer.push_str(&c.to_string());
                };

                if consecutive_back_ticks == 2 {
                    multi_line_code_block = !multi_line_code_block;
                    if !multi_line_code_block {
                        word_buffer.push_str(color_strings::ansi_code(""));
                    };
                    word_buffer.push_str(newline);
                    consecutive_back_ticks = 0;
                };
            }
        
            last_char = c.clone();
        };
        
        // Append any remaining content in word_buffer and line_buffer
        if !word_buffer.is_empty() {
            line_buffer.push_str(&word_buffer);
        }
        if !line_buffer.is_empty() {
            formatted.push_str(&line_buffer);
        }
        
        formatted 
    }
    
    // Initial variables needed for tracking state
    let screen_size: Result<(u16, u16), _> = termion::terminal_size();
    let max_width: u16 = match screen_size {
        Ok(data) => data.0,
        Err(_) => panic!("Could not fetch screen size")
    };
    
    // Preformat the response so that it fits nicely in the 
    // terminal window
    let formatted: String = preformat(ai_response, max_width, &default_color);
    
    // Set the chat speed
    let mut speed: u64 = 25; 
    if formatted.len() > 200 {
        speed = 25; 
    };
   
    // Flag for escape character parsing.
    let mut is_ansi_code: bool = false;

    for c in formatted.chars() {
        
        if c as u8 == 27 && !is_ansi_code {
            is_ansi_code = true;
        } 

        if !is_ansi_code { 
            thread::sleep(Duration::from_millis(speed)); 
        }

        if is_ansi_code && c == 'm' {
            is_ansi_code = false;
        }
        
        handle.write(&[c as u8]).unwrap();
        handle.flush().unwrap(); 
    };
}

