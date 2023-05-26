use termion::raw::{RawTerminal};
use termion::{color, cursor, terminal_size};
use std::io::Stdout;
use std::io::{Write};
use crate::execute_command::command::Command;

pub fn highlight_search_result(stdout: &mut RawTerminal<Stdout>, selected_index: usize, results: &mut Vec<Command>) {
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
                command.cmd,
                color::Fg(color::Reset),
                color::Bg(color::Reset)).as_str();
        } else {
            console_output += format!("({}) - {}\n\r",i,command.cmd).as_str();
        }
    }   
    write_output(stdout, console_output);  
}

pub fn write_output(stdout: &mut RawTerminal<Stdout>, console_output: String) {
    let (_width, height) = terminal_size().unwrap();
    write!(stdout,"{}{}{}{}", 
        termion::clear::All,
        cursor::Goto(1, height - console_output.lines().count() as u16),
        console_output,
        termion::clear::CurrentLine)                        
        .expect("Failed to write to stdout"); 
}

pub fn update_prompt(stdout: &mut RawTerminal<Stdout>, prompt: &str, query: &String){
    let (_width, height) = terminal_size().unwrap();

    write!(stdout, "{}{}{}{}{}{}{}", 
        cursor::Goto(1, height), 
        color::Fg(color::Cyan), 
        prompt, 
        color::Fg(color::Reset),
        query,
        cursor::Goto(prompt.len() as u16 + query.len() as u16 + 1, height),
        termion::cursor::BlinkingBlock)
        .unwrap();    

        stdout.flush().unwrap();
}