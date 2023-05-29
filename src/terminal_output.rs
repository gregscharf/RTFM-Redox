use termion::raw::RawTerminal;
use termion::{color,cursor,terminal_size};
use std::io::Stdout;
use std::io::Write;
use crate::execute_command::command::Command;
use crate::command_variables::variables::Variables;

pub fn highlight_search_result(stdout: &mut RawTerminal<Stdout>, selected_index: usize, results: &mut Vec<Command>) {
    let mut console_output = format!("{}{}Comment: {}\n\r{}{}",
        color::Bg(color::White),
        color::Fg(color::Black),
        results[selected_index].cmnt,
        color::Fg(color::Reset),
        color::Bg(color::Reset));      
    clear_display(stdout);   
    for (i, command) in results.iter().enumerate() {
        if i == selected_index {
            console_output += format!("{}{}({}) - {}{}{}\n\r",
                color::Bg(color::Rgb(165,93,53)),
                color::Fg(color::Rgb(255, 255, 153)),
                command.cmd_id,
                command.cmd,
                color::Fg(color::Reset),
                color::Bg(color::Reset)).as_str();
        } else {
            console_output += format!("({}) - {}\n\r",command.cmd_id,command.cmd).as_str();
        }
    }   
    write_output(stdout, console_output);  
}

pub fn display_selectable_list(stdout: &mut RawTerminal<Stdout>, selectable_list: &mut Vec<Command>){
    clear_display(stdout);
    let mut console_output = format!("{}{}{}{}{}",
        color::Bg(color::White),
        color::Fg(color::Black),
        "Select a result with the up/down arrow keys. Press enter to copy to the clipboard\n\r",
        color::Fg(color::Reset),
        color::Bg(color::Reset));      
              
    for command in selectable_list {
        console_output += format!("({}) - {}\n\r",command.cmd_id,command.cmd).as_str();
    }   
    write_output(stdout, console_output); 
}

pub fn display_command_info(stdout: &mut RawTerminal<Stdout>, command: Command, variables: &mut Variables){
    clear_display(stdout);
    let command_variables = variables.extract_variables_from_command(&command.cmd);
    let mut variable_output = String::new();
    if !command_variables.is_empty() {
        variable_output = format!("\n\r\n\rVariables\n\r----------------------------\n\r{}", command_variables);
    }
    let command_output = format!("Command id: {}\n\rauthor: {}\n\rcomment: {}\n\rcommand: {}\n\r{}\n\r",
        command.cmd_id,                  
        command.author,
        command.cmnt,                  
        command.cmd,
        variable_output);
    write_output(stdout, command_output); 
}

pub fn display_user_variables(stdout: &mut RawTerminal<Stdout>, variables: &mut Variables){
    clear_display(stdout);
    let user_variables = variables.get_printable_variable_list(variables.user_variables.clone());
    if !user_variables.is_empty(){
        let command_output = format!("\n\rUser Variables\n\r----------------------------\n\r{}\n\r",
            user_variables);
        write_output(stdout, command_output); 
    }
}

pub fn display_error(stdout: &mut RawTerminal<Stdout>, error_message: String){
    clear_display(stdout);
    let error_output = format!("{}{}{}\n\r",
            color::Fg(color::Red),
            error_message,
            color::Fg(color::Reset));
    write_output(stdout, error_output); 
}

pub fn display_copy_info(stdout: &mut RawTerminal<Stdout>, command: String){
    let command_output = format!("\n\r\n\rcopied: {}{}{}{}{} to clipboard\n\r",                
        color::Bg(color::Rgb(165,93,53)),
        color::Fg(color::Rgb(255, 255, 153)),
        command,
        color::Fg(color::Reset),
        color::Bg(color::Reset));   
                         
    write_output(stdout, command_output);
}

pub fn write_output(stdout: &mut RawTerminal<Stdout>, console_output: String) {
    let (_width, height) = terminal_size().unwrap();
    if console_output.lines().count() > height as usize {
        display_error(stdout, "Results exceed window height. Keep typing to reduce the number of results.".to_string());
    } else {
        write!(stdout,"{}{}{}\n\r", 
            // termion::clear::All,
            cursor::Goto(1, height - console_output.lines().count() as u16),
            console_output,
            termion::clear::CurrentLine)                        
            .expect("Failed to write to stdout"); 
    }
}

pub fn update_prompt(stdout: &mut RawTerminal<Stdout>, selected_command: &String, current_mode: &String, query: &String){
    let (_width, height) = terminal_size().unwrap();
    let mut prompt: String = format!("{}redOx",
        color::Fg(color::Rgb(165,93,53)));
    if !selected_command.is_empty() {
        prompt = format!("{}redOx[{}{}{}]",
                color::Fg(color::Rgb(165,93,53)),           
                color::Fg(color::Rgb(255,255,71)), 
                selected_command,
                color::Fg(color::Rgb(165,93,53)),
                );
    } 

    let mut mode: String = String::from(":");
    if !current_mode.is_empty() {
        mode = format!("({}{}{}):",
            color::Fg(color::Rgb(204,204,0)), 
            current_mode,
            color::Fg(color::Rgb(165,93,53)));
    }

    write!(stdout, "{}{}{}{}{}{}{}{}", 
        cursor::Goto(1, height), 
        termion::cursor::Left(prompt.len() as u16 + mode.len() as u16 + query.len() as u16 + 1), 
        termion::clear::AfterCursor, 
        prompt,    
        mode,    
        color::Fg(color::Reset),
        query,
        termion::cursor::BlinkingBar)
        .unwrap();    

        stdout.flush().unwrap();
}

pub fn clear_display(stdout: &mut RawTerminal<Stdout>){
    write!(stdout, "{}", 
        termion::clear::All)
        .expect("Failed to write to stdout");       
}