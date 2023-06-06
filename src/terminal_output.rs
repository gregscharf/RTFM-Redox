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

        pub fn display_banner (&mut self) {
            self.clear_display();
                  
            let (_width, height) = terminal_size().unwrap();                  
            let ascii_banner = format!(
                    r#"   
                    ██▀███  ▓█████ ▓█████▄  ▒█████  ▒██   ██▒
                    ▓██ ▒ ██▒▓█   ▀ ▒██▀ ██▌▒██▒  ██▒▒▒ █ █ ▒░
                    ▓██ ░▄█ ▒▒███   ░██   █▌▒██░  ██▒░░  █   ░
                    ▒██▀▀█▄  ▒▓█  ▄ ░▓█▄   ▌▒██   ██░ ░ █ █ ▒ 
                    ░██▓ ▒██▒░▒████▒░▒████▓ ░ ████▓▒░▒██▒ ▒██▒
                    ░ ▒▓ ░▒▓░░░ ▒░ ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ▒▒ ░ ░▓ ░
                        ░▒ ░ ▒░ ░ ░  ░ ░ ▒  ▒   ░ ▒ ▒░ ░░   ░▒ ░
                        ░░   ░    ░    ░ ░  ░ ░ ░ ░ ▒   ░    ░  
                        ░        ░  ░   ░        ░ ░   ░    ░  
                                        ░                        
                "#,
                );

                for (line_number,line) in ascii_banner.lines().enumerate() {
                    let trimmed_line = line.trim_start();
                    let cursor_y = (height - 13) + line_number as u16;
                    let mut cursor_x = 1;

                    if line_number == 1 { //fix for top of first character line
                        cursor_x = 2;
                    }

                    write!(
                        self.stdout,
                        "{}{}{}{}{}",
                        cursor::Goto(cursor_x, cursor_y as u16),
                        color::Fg(color::Red),
                        trimmed_line,
                        color::Fg(color::Reset),
                        cursor::Goto(cursor_x, 1) // Reset cursor position for next line
                    ).expect("Failed to write to stdout");
                }
                self.write_output(format!("{}{}{}{}\n\r{}\n\r{}{}",
                cursor::Goto(1, height - 1),
                color::Fg(color::Rgb(255, 255, 153)),
                color::Fg(color::Rgb(255, 255, 153)),
                "For help type 'help'",
                "Ctrl+r to start searcing for commands",
                color::Bg(color::Reset),
                color::Fg(color::Reset)));
            
        }            
     

        pub fn highlight_search_result(&mut self, selected_index: usize, results: Vec<Command>) {
            self.clear_display();  

            let (width, height) = terminal_size().unwrap();
            let available_height = height as usize - 10;
            let mut start_index = 0;
            let mut end_index = results.len()- 1;

            if results.len() > available_height { //results won't fit on screen
                if selected_index < available_height { //nothing more to scroll
                    start_index = 0;
                    end_index = available_height;
                } else { //scroll results
                    start_index = selected_index - available_height;
                    end_index = selected_index;
                }
            }

            let mut console_output = format!("{}{}Comment: {}\n\r{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                results[selected_index].cmnt,
                color::Fg(color::Reset),
                color::Bg(color::Reset));

            for (i, command) in results.iter().enumerate() {
                if i >= start_index && i <= end_index {
                    if i == selected_index {
                        console_output += format!("{}{}({}) - {:.maxwidth$}{}{}\n\r",
                            color::Bg(color::Rgb(165,93,53)),
                            color::Fg(color::Rgb(255, 255, 153)),
                            command.cmd_id,
                            command.cmd,
                            color::Fg(color::Reset),
                            color::Bg(color::Reset),
                            maxwidth = width as usize - 9).as_str();
                    } else {
                        console_output += format!("({}) - {:.maxwidth$}\n\r",command.cmd_id,command.cmd, maxwidth = width as usize - 9).as_str();
                    }
                }
            }

            console_output += format!("\n\rResults: {}{}{} / {}\n\r{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                selected_index + 1,
                results.len(),
                color::Fg(color::Reset),
                color::Bg(color::Reset)).as_str(); 

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

            let (width, height) = terminal_size().unwrap();
            let available_height = height as usize - 10;

            let mut start_index = 0;
            let mut end_index = selectable_list.len() - 1;

            if selectable_list.len() > available_height {
                start_index = selectable_list.len() - available_height - 1;
                end_index = selectable_list.len() - 1;
            }

            for (i, command) in selectable_list.iter().enumerate() {
                if i >= start_index && i <= end_index {
                    console_output += format!("({}) - {:.maxwidth$}\n\r",
                        command.cmd_id,
                        command.cmd,
                        maxwidth = width as usize - 9).as_str();
                }
            }

            console_output += format!("\n\rResults: {}{}{} / {}\n\r{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                start_index + 1,
                selectable_list.len(),
                color::Fg(color::Reset),
                color::Bg(color::Reset)).as_str();

            self.write_output(console_output); 
        }

        pub fn display_command_info(&mut self, command: Command, variables: &mut Variables){
            let command_variables = variables.extract_variables_from_command(&command.cmd);
            let mut variable_output = String::new();

            if !command_variables.is_empty() {
                variable_output = format!("\n\rVariables \n\r----------------------------\n\r{}", command_variables);
            }

            let mut command_references = String::new();
            if !command.references.is_empty() {
                command_references = format!("{}references{} :\n\r",
                    color::Fg(color::Rgb(165,93,53)),
                    color::Fg(color::Reset));
                for reference in &command.references {
                    command_references += &format!("{}\n\r", reference.ref_value);
                }
            }

            let min_width = 11;

            let command_output = format!("{}{:<width$}{}: {}\n\r{}{:<width$}{}: {}\n\r{}{:<width$}{}: {}\n\r{}{:<width$}{}: {}\n\r{}\n\r{}\n\r ",
                color::Fg(color::Rgb(165,93,53)),
                String::from("Command id"),
                color::Fg(color::Reset),
                command.cmd_id,    
                color::Fg(color::Rgb(165,93,53)),
                String::from("author"),   
                color::Fg(color::Reset),           
                command.author,
                color::Fg(color::Rgb(165,93,53)),
                String::from("comment"),  
                color::Fg(color::Reset),
                command.cmnt,    
                color::Fg(color::Rgb(165,93,53)),
                String::from("command"),     
                color::Fg(color::Reset),      
                command.cmd,
                command_references,
                variable_output,
                width = min_width);
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