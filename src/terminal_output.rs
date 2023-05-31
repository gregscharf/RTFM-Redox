pub mod output {
    use termion::{color,cursor,terminal_size};
    use std::io::{self, stdout, Write};
    use termion::raw::IntoRawMode;
    use crate::command_variables::variables::Variables;
    use crate::database::command_table::Command;

    pub struct Output {
        stdout: termion::raw::RawTerminal<io::Stdout>,
    }

    impl Output {

        pub fn new() -> Self {
            let stdout = stdout().into_raw_mode().unwrap();
            Output { stdout }
        }


        pub fn highlight_search_result(&mut self, selected_index: usize, results: &mut Vec<Command>) {
            let mut console_output = format!("{}{}Comment: {}\n\r{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                results[selected_index].cmnt,
                color::Fg(color::Reset),
                color::Bg(color::Reset));

            self.clear_display();   

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
            self.write_output(console_output);  
        }

        pub fn display_selectable_list(&mut self, selectable_list: &mut Vec<Command>){
            self.clear_display();
            let mut console_output = format!("{}{}{}{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                "Select a result with the up/down arrow keys. Press enter to copy to the clipboard\n\r",
                color::Fg(color::Reset),
                color::Bg(color::Reset));      
                    
            for command in selectable_list {
                console_output += format!("({}) - {}\n\r",command.cmd_id,command.cmd).as_str();
            }   
            self.write_output(console_output); 
        }

        pub fn display_command_info(&mut self, command: Command, variables: &mut Variables){
            // clear_display(stdout);
            let command_variables = variables.extract_variables_from_command(&command.cmd);
            let mut variable_output = String::new();
            if !command_variables.is_empty() {
                variable_output = format!("\n\r\n\rVariables\n\r----------------------------\n\r{}", command_variables);
            }
            let command_output = format!("Command id: {}\n\rauthor: {}\n\rcomment: {}\n\rcommand: {}\n\r{}\n\r ",
                command.cmd_id,                  
                command.author,
                command.cmnt,                  
                command.cmd,
                variable_output);
            self.write_output(command_output); 
        }

        pub fn display_user_variables(&mut self, variables: &mut Variables){
            self.clear_display();
            let user_variables = variables.get_printable_variable_list(variables.user_variables.clone());
            if !user_variables.is_empty(){
                let command_output = format!("\n\rUser Variables\n\r----------------------------\n\r{}\n\r",
                    user_variables);
                self.write_output(command_output); 
            }
        }

        pub fn display_error(&mut self, error_message: String){
            self.clear_display();
            let error_output = format!("{}{}{}\n\r",
                    color::Fg(color::Red),
                    error_message,
                    color::Fg(color::Reset));
            self.write_output(error_output); 
        }

        pub fn display_copy_info(&mut self, command: String){
            let command_output = format!("copied: {}{}{}{}{} to clipboard\n\r",  
                color::Bg(color::Rgb(165,93,53)),
                color::Fg(color::Rgb(255, 255, 153)),
                command,
                color::Fg(color::Reset),
                color::Bg(color::Reset));   
                                
            self.write_output(command_output);
        }

        pub fn write_output(&mut self, console_output: String) {
            let (_width, height) = terminal_size().unwrap();
            if console_output.lines().count() > height as usize {
                self.display_error("Results exceed window height. Keep typing to reduce the number of results.".to_string());
            } else {
                write!(self.stdout,"{}{}{}\n\r", 
                    // termion::clear::All,
                    cursor::Goto(1, height - console_output.lines().count() as u16 + 1),
                    console_output,
                    termion::clear::CurrentLine)                        
                    .expect("Failed to write to stdout"); 
            }
        }

        pub fn update_prompt(&mut self, selected_command: &String, current_mode: &String, query: &String){
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

            write!(self.stdout, "{}{}{}{}{}{}{}{}", 
                cursor::Goto(1, height), 
                termion::cursor::Left(prompt.len() as u16 + mode.len() as u16 + query.len() as u16 + 1), 
                termion::clear::AfterCursor, 
                prompt,    
                mode,    
                color::Fg(color::Reset),
                query,
                termion::cursor::BlinkingBar)
                .unwrap();    

                self.stdout.flush().unwrap();
        }

        pub fn clear_display(&mut self,){
            // let mut stdout = stdout().into_raw_mode().unwrap();
            write!(self.stdout, "{}", 
                termion::clear::All)
                .expect("Failed to write to stdout");       
        }
    }
}