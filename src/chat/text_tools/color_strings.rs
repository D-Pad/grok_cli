
#[derive(Debug)]
pub enum Color {
    Black, 
    Red,
    Green, 
    Yellow, 
    Blue,
    Magenta,
    Cyan,
    White,
    NoCol,
}

pub enum TextStyle {
    Plain,
    Bold,
    Italic,
    Underline,
}

pub struct FormattedText {
    pub color: Color, 
    pub text: String
}


impl FormattedText {
    
    pub fn new(color: &str, text: String) -> Self {
        
        let color_enum = match color.to_lowercase().as_str() {
            "black" => Color::Black, 
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "none" => Color::NoCol,
            _ => Color::White,
        };
        
        FormattedText { 
            color: color_enum, 
            text: text 
        } 
    
    }

    pub fn dye(&self, color: &str) -> String {
        let escape_char: &str = ansi_code(color);
        let clear: &str = ansi_code("");
        format!("{}{}{}", escape_char, &self.text, clear) 
    }

}

pub fn ansi_code(input_str: &str) -> &'static str {
    match input_str {
        "black" => "\x1b[30m",
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        
        "!black" => "\x1b[40m",
        "!red" => "\x1b[41m",
        "!green" => "\x1b[42m",
        "!yellow" => "\x1b[43m",
        "!blue" => "\x1b[44m",
        "!magenta" => "\x1b[45m",
        "!cyan" => "\x1b[46m",
        "!white" => "\x1b[47m",
        
        "bold" => "\x1b[1m",
        "italic" => "\x1b[3m",
        "underline" => "\x1b[4m",
        
        _ => "\x1b[0m", 
    }
}

