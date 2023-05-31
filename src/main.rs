mod terminal_output;
mod terminal_commands;
use terminal_commands::execute_command;
use database::command_table;
use sqlx::Error;
mod command_variables;
mod database;
use termion::event::Key;
use termion::input::TermRead;
use clipboard::{ClipboardContext,ClipboardProvider};

#[tokio::main]
async fn main() {

    let mut database: database::database::Database = match database::database::Database::new().await {
        Ok(database) => database,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let terminal_output = &mut terminal_output::output::Output::new();
   
    // Initialize the search query and search results
    let mut query: String = String::new();
    let mut command_results: Vec<command_table::Command> = Vec::new();
    let mut search_mode: bool = false;
    
    let mut variables = command_variables::variables::Variables::new();

    //Create a history for selected commands
    let mut command_history: Vec<command_table::Command> = Vec::new();
    let mut selected_command_in_history: usize = 0;
    let mut history_mode: bool = false;

    let mut selected_result_index: usize = 0;
    let mut results_selection_mode: bool = false;

    //Print help when application starts
    terminal_output.clear_display();
    let command_output: String = execute_command(&"help".to_string()).await;
    terminal_output.write_output( command_output);

    //TODO: Add a cleaner, more consistent method for parsing and executing commands.
    //Move most of this into terminal_commands.rs
    loop {
        let mut selected_command: String = String::from("");
        let mut current_mode: String = String::from("");
        if command_history.len() > 0 {
            selected_command =  command_history[selected_command_in_history].cmd_id.to_string();
        }

        if search_mode { 
            current_mode = "find".to_string();
        } else if history_mode {
            current_mode = "history".to_string();
        }
        terminal_output.update_prompt(&selected_command, &current_mode, &query);

        let key = std::io::stdin().keys().next().unwrap();
        match key {          
            Ok(Key::Ctrl('r')) => { // Ctrl + R to enter search mode and query the database as you type
                search_mode = true;
                query.clear();
            }
            Ok(Key::Backspace) => {
                query.pop();             
                terminal_output.update_prompt( &selected_command, &current_mode, &query);
                if search_mode  {
                    command_results.clear(); 
                    results_selection_mode = false;                      
                    
                    if query.len() > 0 { 
                        // let command = format!("search {}", query);
                        // results = search_commands(&db, &mut stdout, &command).await;
                        // command_results = database.search_commands(&query).await;

                        let db_command = format!("search {}", query);
                        command_results = database.search_commands(&db_command).await;

                        if !command_results.is_empty() {
                            terminal_output.display_selectable_list(&mut command_results);
                        } else {
                            terminal_output.display_error(String::from("Not Found"));
                        }
                    } else {
                        let command_output: String = execute_command(&"help".to_string()).await;
                        terminal_output.write_output( command_output);                        
                    }                        
                } 
            }
            Ok(Key::Up) => {  // Move up in results  
                if command_results.len() > 0 { 
                    if results_selection_mode == false {
                        results_selection_mode = true;
                        selected_result_index = command_results.len() - 1;
                    } else {             
                        selected_result_index = (selected_result_index + command_results.len() - 1) % command_results.len();
                    }
                    terminal_output.highlight_search_result(selected_result_index, &mut command_results); 
                }
            }
            Ok(Key::Down) => {// Move down in results             
                if command_results.len() > 0 {
                    if results_selection_mode == false {
                        results_selection_mode = true;
                        selected_result_index = 0;
                    } else { 
                        selected_result_index = (selected_result_index + 1) % command_results.len();
                    }
                    terminal_output.highlight_search_result(selected_result_index, &mut command_results);
                }
            }   

            Ok(Key::Ctrl('u')) => {// Url encode and then copy text to the clipboard             
                if command_history.len() > 0 {
                    terminal_output.clear_display();
                    let command = variables.replace_variables_in_command(&command_results[selected_result_index].cmd);
                    let mut encoded = String::new();                
                    for byte in command.bytes() {
                        match byte {
                            // Alphanumeric characters and a few special characters are not encoded
                            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                                encoded.push(byte as char);
                            }
                            // Percent-encoded all other characters
                            _ => {
                                encoded.push('%');
                                encoded.push_str(&format!("{:02X}", byte));
                            }
                        }
                    }
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(encoded.clone()).unwrap();                                        
                    terminal_output.display_command_info( command_history[selected_command_in_history].clone(), &mut variables);
                    terminal_output.display_copy_info( encoded);
                } else {
                    terminal_output.display_error( String::from("There isn't a command currently selected."));
                }                  
            },             
            Ok(Key::Ctrl('c')) => {// Copy the current command to the clipboard            
                if command_history.len() > 0 {
                    terminal_output.clear_display();
                    let command = variables.replace_variables_in_command(&command_results[selected_result_index].cmd);
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command.clone()).unwrap();                                        
                    terminal_output.display_command_info( command_history[selected_command_in_history].clone(), &mut variables);
                    terminal_output.display_copy_info( command);
                } else {
                    terminal_output.display_error( String::from("There isn't a command currently selected."));
                }                  
            },                     
            Ok(Key::Ctrl('v')) => {// Paste text from the clipboard, needed for adding content to the database            
                let mut clipboard = ClipboardContext::new().unwrap();               
                if let Ok(text) = clipboard.get_contents() {
                    query.push_str(text.as_str());
                    let command = format!("{}", query);
                    terminal_output.write_output( command); 
                } 
            },            
            Ok(Key::Char(c)) if c != '\n'  => { 
                results_selection_mode = false;               
                query.push(c);
                terminal_output.update_prompt( &selected_command, &current_mode, &query);
                if search_mode {
                    if query.len() > 0 {
                        command_results.clear();
                        let db_command = format!("search {}", query);
                        command_results = database.search_commands(&db_command).await;

                        if !command_results.is_empty() {
                            terminal_output.display_selectable_list(&mut command_results);
                        } else {
                            terminal_output.display_error(String::from("Not Found"));
                        }
                    }
                }
            }            
            Ok(Key::Ctrl('q')) => {// Exit the CLI
                break;
            }
            Ok(Key::Esc)  => {
                search_mode = false;
                history_mode = false;
                results_selection_mode = false;
                query.clear();
                terminal_output.clear_display();
                let command_output: String = execute_command(&"help".to_string()).await;
                terminal_output.write_output(command_output);
            }
            Ok(Key::Ctrl('h')) => {// Display selectable list of past commands
                if command_history.len() > 0 {
                    terminal_output.clear_display();
                    command_results.clear();
                    terminal_output.display_selectable_list(&mut command_history);
                    command_results = command_history.clone();
                    results_selection_mode = false;
                    history_mode = true;
                } else {
                    terminal_output.display_error(String::from("History is currently empty."));
                }                
            }            
            Ok(Key::Char('\n')) => {
                let command_output: String;
                history_mode = false;
                if query.starts_with("add") {
                    if query.contains(" -c "){
                        let result = database.add_command(&query).await;
                        match result {
                            Ok(command_output) => {
                                // TODO: add the newly added command to results and set as selected
                                // terminal_output.display_command_info(command_history[selected_command_in_history].clone(), &mut variables);
                                terminal_output.write_output(command_output);
                            }
                            Err(sqlx_error) => {
                                match sqlx_error {
                                    Error::RowNotFound => {
                                        println!("Row not found error occurred");
                                    }
                                    _ => {
                                        println!("Other SQLx error occurred");
                                    }
                                }
                            }
                        }    

                   } else {
                        command_output = String::from("To add a command to the database you must include -c followed by the command\n\r-d with a description of the command is optional"); 
                        terminal_output.clear_display();
                        terminal_output.write_output( command_output); 
                   }
 
                } else if query.starts_with("update") { //update command
                    terminal_output.clear_display();
                    let result = database.update_command(&query, &mut command_history[selected_command_in_history]).await;
                    match result {
                        Ok(command_output) => {
                            terminal_output.display_command_info(command_history[selected_command_in_history].clone(), &mut variables);
                            terminal_output.write_output(command_output);
                        }
                        Err(sqlx_error) => {
                            match sqlx_error {
                                Error::RowNotFound => {
                                    terminal_output.display_error(String::from("Check that you've entered a correct column name"));
                                }
                                _ => {
                                    terminal_output.display_error(String::from("Check that you've entered a correct column name"));
                                }
                            }
                        }
                    }                
                } else if results_selection_mode == true { //pressed enter while arrowing through selectable list
                    let command = variables.replace_variables_in_command(&command_results[selected_result_index].cmd);
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command.clone()).unwrap();
                    //Add new command to command_history if it isn't already in the command_history
                    //Also set the index
                    let mut add_to_history: bool = true;
                    let commands_slice: &[command_table::Command] = &command_history;
                    for (i, command) in commands_slice.iter().enumerate(){
                        if command.cmd_id == command_results[selected_result_index].cmd_id.to_owned(){
                            add_to_history = false;
                            selected_command_in_history = i;
                            break;
                        }
                    }
                    if add_to_history {
                        command_history.push(command_results[selected_result_index].clone());
                        selected_command_in_history = command_history.len() - 1;
                    }
                    terminal_output.clear_display();
                    terminal_output.display_command_info( command_results[selected_result_index].clone(), &mut variables);
                    terminal_output.display_copy_info( command);
                    results_selection_mode = false;
                    search_mode = false;                    
                } else if query.starts_with("hist") { //history command
                    terminal_output.clear_display();
                    if command_history.len() > 0 {
                        command_results.clear();
                        terminal_output.display_selectable_list(&mut command_history);
                        command_results = command_history.clone();
                        results_selection_mode = false;
                    } else {
                        terminal_output.display_error( String::from("History is currently empty."));
                    }
                } else if query.starts_with("env"){ //show user set variables
                    terminal_output.display_user_variables( &mut variables);    
                } else if query.starts_with("set") { //set variables
                    terminal_output.clear_display();
                    let query_values: Vec<&str> = query.split_whitespace().collect();
                    if let Some(variable) = query_values.get(1) {
                        if let Some(value) = query_values.get(2) {
                            variables.set_user_variable(variable.to_string(), value.to_string());
                        } else {
                            terminal_output.display_error( "You must supply a value".to_string());
                        }

                        if command_history.len() > 0 {                                     
                            terminal_output.display_command_info( command_history[selected_command_in_history].clone(), &mut variables);
                        }

                        //copy the current command to the clipboard again after setting a variable
                        let command = variables.replace_variables_in_command(&command_history[selected_command_in_history].cmd);
                        let mut clipboard = ClipboardContext::new().unwrap();
                        clipboard.set_contents(command.clone()).unwrap();
                        terminal_output.display_copy_info( command);
                    } else {
                        terminal_output.display_error( "You must supply a variable".to_string());
                    }
                } else if query.starts_with("info") { //info command
                    terminal_output.clear_display();
                    if command_history.len() > 0 {
                        terminal_output.display_command_info( command_history[selected_command_in_history].clone(), &mut variables)
                    } else {
                        terminal_output.display_error( String::from("There isn't a command currently selected."));
                    }                            
                } else if query.starts_with("search") {
                    // results = search_commands(&db, &mut stdout, &query).await;
                    command_results = database.search_commands(&query).await;
                    
                } else if query.starts_with("help") {
                    if query.contains("add") {
                        command_output = String::from("To add a command to the database\n\r'add -c command [optional: -d comment]"); 
                    } else if query.contains("search"){
                        command_output = String::from("Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rEsc\t\t Exit search mode.\n\rOr use 'search' command followed by a term to search results.");             
                    } else {
                        command_output = String::from("Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rCtrl+c\t\t Copy currently selected command to clipboard.\n\rCtrl+u\t\t URL-encode and then copy currently selected command to clipboard.\n\rCrtl+h or hist\t Display selectable history of already selected commands.\n\rCtrl+v\t\t Paste from clipboard\n\rinfo\t\t Display info on the currently selected command.\n\renv\t\t Show currently set user variables\n\radd\t\t Add a command to the database e.g. 'add -c stty raw -echo;fg'\n\rupdate\t\t update database columns of currently selected command e.g. 'update comment bash reverse shell'.\n\rEsc\t\t Exit current mode.\n\rhelp\t\t Display help\n\rCtrl+q or exit\t Exit redOx.\n\r"); 
                    }   
                    terminal_output.clear_display();
                    terminal_output.write_output( command_output);                    
                } else if query.starts_with("exit") {
                    break;
                } else {
                    terminal_output.clear_display();
                    command_output = execute_command(&query).await;
                    terminal_output.write_output( command_output);
                }                
                query.clear();
            }
            _ => {}
        }
    }
}

