use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, terminal_size};
use std::io::Stdout;
use std::io::{Write};

pub fn highlight_search_result(stdout: &mut RawTerminal<Stdout>, selected_index: usize, height: u16, results: &mut Vec<String>) {
    let mut console_output = format!("{}{}{}{}{}",
        color::Bg(color::White),
        color::Fg(color::Black),
        "Select a result with the up/down arrow keys. Press enter to copy to the clipboard\n\r",
        color::Fg(color::Reset),
        color::Bg(color::Reset));      
              
    for (i, command) in results.iter().enumerate() {
        if i == selected_index {
            console_output += format!("{}{}({}) - {}{}{}\n\r",
                color::Bg(color::Cyan),
                color::Fg(color::Black),
                i,
                command,
                color::Fg(color::Reset),
                color::Bg(color::Reset)).as_str();
        } else {
            console_output += format!("({}) - {}\n\r",i,command).as_str();
        }
    }   
    write_output(stdout, height, console_output);  
}

pub fn write_output(stdout: &mut RawTerminal<Stdout>, height: u16, console_output: String) {
    write!(stdout,"{}{}{}{}", 
        termion::clear::All,
        cursor::Goto(1, height - console_output.lines().count() as u16),
        console_output,
        termion::clear::CurrentLine)                        
        .expect("Failed to write to stdout"); 
}