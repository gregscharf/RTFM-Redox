mod terminal_output;
mod terminal_actions;
use terminal_actions::{execute_search, execute_help};
mod search;
use sqlx::Error;
mod command_variables;
use crate::command_variables::variables::Variables;
mod database;
use crate::database::database::Database;
use termion::event::Key;
use termion::input::TermRead;
use clipboard::{ClipboardContext,ClipboardProvider};

#[tokio::main]
async fn main() {

    let mut database: Database = match Database::new().await {
        Ok(database) => database,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    // All output to the terminal
    let terminal_output = &mut terminal_output::output::Output::new();
   
    // Initialize the search results and selected result history
    let mut search_results = search::search::Results::new();

    // Initialize the search query 
    let mut query: String = String::new();
    
    // User set variables and current command variables
    let mut variables: Variables = Variables::new();

    // Variable that affect the dynamic prompt
    let mut selected_command_id: String;
    let mut current_mode: String;

    //Clear screen and print banner and truncated help when application starts
    let formatted_banner = terminal_output.get_banner_speedy();
    terminal_output.display_banner(formatted_banner);    

    loop {

        // Build the command prompt
        match search_results.get_current_command() {
            Some(command) => {
                selected_command_id = command.cmd_id.to_string()
            }
            None => {
                selected_command_id = String::from("");
            }
        }
        current_mode = search_results.get_current_mode();
        terminal_output.update_prompt(selected_command_id.clone(), current_mode.clone(), &query);

        // Capture user input
        let key = std::io::stdin().keys().next().unwrap();
        match key {          
            Ok(Key::Ctrl('r')) => { // Ctrl + R to enter search mode and query the database as you type
                search_results.cycle_search_mode();
                query.clear();
            }
            Ok(Key::Backspace) => {
                query.pop();             
                terminal_output.update_prompt( selected_command_id.clone(), current_mode.clone(), &query);
                
                if search_results.get_search_mode() != search::search::OFF {

                    terminal_output.clear_display();
                    
                    if query.len() > 0 { 
                        match execute_search(search_results.clone().get_search_column(), format!("search {}", query), &mut database).await {
                            Some(results) => {
                                search_results.set_results(results);
                            } 
                            _ => {}
                        }
                    } else {
                        let formatted_banner = terminal_output.get_banner_speedy();
                        terminal_output.display_banner(formatted_banner);  
                    }                        
                } 
            }
            Ok(Key::Up) => {  // Move up in results  
                match search_results.cycle_through_results(search::search::UP) {
                    Ok(_commands) => {
                        search_results.highlight_current_selection(terminal_output);
                    }
                    Err(error) => {
                        terminal_output.display_error(error.to_string());
                    }
                }
            }
            Ok(Key::Down) => {// Move down in results  
                match search_results.cycle_through_results(search::search::DOWN) {
                    Ok(_commands) => {
                        search_results.highlight_current_selection(terminal_output);
                    }
                    Err(error) => {
                        terminal_output.display_error(error.to_string());
                    }
                }
            }   
            Ok(Key::Ctrl('u')) => {// Url encode and then copy text to the clipboard   

                let command_history = search_results.get_history();          
                if command_history.len() > 0 {
                    terminal_output.clear_display();

                    let command = search_results.get_current_command().unwrap();
                    
                    let encoded = command.clone().url_encode(variables.clone());

                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(encoded.clone()).unwrap();   

                    terminal_output.display_command_info( command, &mut variables);

                    terminal_output.display_copy_info( encoded);
                } else {
                    terminal_output.display_error( String::from("There isn't a command currently selected."));
                }                  
            },             
            Ok(Key::Ctrl('c')) => {// Copy the current command to the clipboard      
                let command_history = search_results.get_history();         
                if command_history.len() > 0 {
                    terminal_output.clear_display();

                    let command = search_results.get_current_command().unwrap();
                    
                    let command_syntax = command.clone().set_command_variables(variables.clone());

                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command_syntax.clone()).unwrap();   

                    terminal_output.display_command_info( command, &mut variables);

                    terminal_output.display_copy_info( command_syntax);
                } else {
                    terminal_output.display_error( String::from("There isn't a command currently selected."));
                }                  
            }           
            Ok(Key::Ctrl('v')) => {// Paste text from the clipboard            
                let mut clipboard = ClipboardContext::new().unwrap();               
                if let Ok(text) = clipboard.get_contents() {
                    query.push_str(text.as_str());
                    let command = format!("{}", query);
                    terminal_output.write_output( command); 
                } 
            },            
            Ok(Key::Char(c)) if c != '\n'  => { 
                query.push(c);

                terminal_output.update_prompt( selected_command_id.clone(), current_mode.clone(), &query);

                if search_results.get_search_mode() != search::search::OFF {
                    if query.len() > 0 {
                        match execute_search(search_results.clone().get_search_column(), format!("search {}", query), &mut database).await {
                            Some(results) => {
                                search_results.set_results(results);
                            } 
                            _ => {}
                        }                    
                    }
                }
            }            
            Ok(Key::Ctrl('q')) => {// Exit the CLI
                break;
            }
            Ok(Key::Esc)  => {
                search_results.reset();

                query.clear();

                execute_help("".to_string()).await;                                        
            }
            Ok(Key::Ctrl('h')) => {// Display selectable list of commands from history
                let command_history = search_results.get_history();
                if command_history.len() > 0 {
                   
                    search_results.set_results(command_history.clone());

                    terminal_output.clear_display();
                    terminal_output.display_selectable_list(&mut command_history.clone());

                    search_results.set_history_mode(true);
                } else {
                    terminal_output.display_error(String::from("History is currently empty."));
                }                
            }            
            Ok(Key::Char('\n')) => { // Enter key has been pressed, check for valid commands
                let command_output: String;

                search_results.set_history_mode(false);

                if query.starts_with("add") {
                    if query.contains(" -c "){
                        let result = database.add_command(&query).await;
                        match result {
                            Ok(commands) => {
                                search_results.set_results(commands);

                                let command = search_results.get_current_command().unwrap();

                                let command_syntax = variables.clone().replace_variables_in_command(&command.clone().cmd);
                                
                                let mut clipboard = ClipboardContext::new().unwrap();
                                clipboard.set_contents(command_syntax.clone()).unwrap();
                                
                                search_results.add_command_to_history(command.clone());

                                terminal_output.clear_display();
                                terminal_output.display_command_info( command, &mut variables);
                                terminal_output.display_copy_info( command_syntax);
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
 
                } else if query.starts_with("update") { //update a db column for the current command
                    terminal_output.clear_display();

                    let history_command = search_results.get_current_history_command();
                    match history_command {
                        Some(mut command) => {
                            let result = database.update_command(&query, &mut command).await;

                            match result {
                                Ok(update_message) => {                                    
                                    //TODO: get the command to update automatically without another hacky history add
                                    search_results.add_command_to_history(command.clone());

                                    terminal_output.display_command_info( command, &mut variables);
                                    terminal_output.write_output(update_message);
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
                        }
                        None => {
                            terminal_output.display_error("History is currently empty.".to_string());
                        }
                    }
                } else if search_results.get_results_selection_mode() { //pressed enter while arrowing through selectable list
                    let mut command = search_results.get_current_command().unwrap();
                    
                    search_results.add_command_to_history(command.clone());

                    let command_syntax = command.set_command_variables(variables.clone());

                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command_syntax.clone()).unwrap();

                    terminal_output.clear_display();
                    terminal_output.display_command_info( command, &mut variables);
                    terminal_output.display_copy_info( command_syntax);

                    search_results.set_search_mode(search::search::OFF);                 
                } else if query.starts_with("hist") { //history command
                    terminal_output.clear_display();

                    let command_history = search_results.get_history();
                    if command_history.len() > 0 {
                        terminal_output.display_selectable_list(&mut command_history.clone());

                        search_results.set_results(command_history);
                    } else {
                        terminal_output.display_error( String::from("History is currently empty."));
                    }
                } else if query.starts_with("set") { //set variables
                    terminal_output.clear_display();
 
                    let query_values: Vec<&str> = query.split_whitespace().collect();
                    if let Some(variable) = query_values.get(1) {
                        if let Some(value) = query_values.get(2) {
                            variables.set_user_variable(variable.to_string(), value.to_string());
                        } else {
                            terminal_output.display_error( "You must supply a value".to_string());
                        }
                        //FIX: Code below duplicated in "info" command
                        let results = search_results.get_selected_history_command();
                        match results {
                            Ok(command) => {
                                terminal_output.display_command_info( command.clone(), &mut variables);

                                let command_syntax = variables.replace_variables_in_command(&command.cmd.to_string());

                                let mut clipboard = ClipboardContext::new().unwrap();
                                clipboard.set_contents(command_syntax.clone()).unwrap();
        
                                terminal_output.display_copy_info( command_syntax);                                
                            }
                            Err(error) => {
                                terminal_output.display_error(error.to_string());
                            }
                        }   
                    } else {
                        terminal_output.display_error( "You must supply a variable".to_string());
                    }
                } else if query.starts_with("info") { //info command
                    terminal_output.clear_display();
  
                    let results = search_results.get_selected_history_command();
                    match results {
                        Ok(command) => {
                            terminal_output.display_command_info( command.clone(), &mut variables);

                            let command_syntax = variables.replace_variables_in_command(&command.cmd.to_string());
                            
                            let mut clipboard = ClipboardContext::new().unwrap();
                            clipboard.set_contents(command_syntax.clone()).unwrap();
    
                            terminal_output.display_copy_info( command_syntax);                                
                        }
                        Err(_error) => {
                            terminal_output.display_error( String::from("There isn't a command currently selected."));
                        }
                    }   
                } else if query.starts_with("search") { 
                    let column = "Cmd";
                    match execute_search(column.to_string(), format!("{}", query), &mut database).await {
                        Some(results) => {
                            search_results.set_results(results);
                        } 
                        _ => {}
                    }
                } else if query.starts_with("help") {
                    execute_help(query.clone()).await;    
                } else if query.starts_with("env"){ //show user set variables                   
                    terminal_output.display_user_variables( &mut variables);                        
                } else if query.starts_with("exit") {
                    break;
                } else {
                    terminal_output.clear_display();
                    terminal_output.display_error("Command not found.".to_string());
                }                
                query.clear();
            }
            _ => {}
        }
    }
}

