mod terminal_output;
use terminal_output::{highlight_search_result,write_output,update_prompt,display_selectable_list,display_error,display_command_info, display_user_variables, clear_display, display_copy_info};
mod execute_command; 
use execute_command::{execute_command,search_commands, execute_update_command,command};
mod command_variables;
use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
use std::io::stdout;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use clipboard::{ClipboardContext,ClipboardProvider};

const DB_URL: &str = "sqlite://snips.db";

#[tokio::main]
async fn main() {

    // Open a connection to the database
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Initialize the search query and search results
    let mut query: String = String::new();
    let mut results: Vec<command::Command> = Vec::new();
    let mut search_mode: bool = false;
    
    let mut variables = command_variables::variables::Variables::new();

    //Create a history for selected commands
    let mut command_history: Vec<command::Command> = Vec::new();
    let mut selected_command_in_history: usize = 0;
    let mut history_mode: bool = false;
    // Set up the scrolling output buffer
    // let mut output: Vec<String> = vec![];
    // let mut total_output: i32 = 1;
    // let mut output_start = 0;

    let mut selected_result_index: usize = 0;
    let mut results_selection_mode: bool = false;

    clear_display(&mut stdout);
    let command_output: String = execute_command(&db, &"help".to_string()).await;
    write_output(&mut stdout, command_output);

    //TODO: Add a cleaner, more consistent method for parsing and executing commands.
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
        update_prompt(&mut stdout, &selected_command, &current_mode, &query);

        let key = std::io::stdin().keys().next().unwrap();
        match key {          
            Ok(Key::Ctrl('r')) => { // Ctrl + R to enter search mode and query the database as you type
                search_mode = true;
                query.clear();
            }
            Ok(Key::Backspace) => {
                query.pop();             
                update_prompt(&mut stdout, &selected_command, &current_mode, &query);
                if search_mode  {
                    results.clear(); 
                    results_selection_mode = false;                      
                    
                    if query.len() > 0 { 
                        let command = format!("search {}", query);
                        results = search_commands(&db, &mut stdout, &command).await;
                    } else {
                        let command_output: String = execute_command(&db, &"help".to_string()).await;
                        write_output(&mut stdout, command_output);                        
                    }                        
                } 
            }
            Ok(Key::Up) => {  // Move up in results  
                if results.len() > 0 { 
                    if results_selection_mode == false {
                        results_selection_mode = true;
                        selected_result_index = results.len() - 1;
                    } else {             
                        selected_result_index = (selected_result_index + results.len() - 1) % results.len();
                    }
                    highlight_search_result(&mut stdout,selected_result_index, &mut results); 
                }
            }
            Ok(Key::Down) => {// Move down in results             
                if results.len() > 0 {
                    if results_selection_mode == false {
                        results_selection_mode = true;
                        selected_result_index = 0;
                    } else { 
                        selected_result_index = (selected_result_index + 1) % results.len();
                    }
                    highlight_search_result(&mut stdout,selected_result_index, &mut results);
                }
            }   
            Ok(Key::Ctrl('c')) => {// Paste text from the clipboard, needed for adding content to the database            
                if command_history.len() > 0 {
                    let command = variables.replace_variables_in_command(&results[selected_result_index].cmd);
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command.clone()).unwrap();                                        
                    display_command_info(&mut stdout, command_history[selected_command_in_history].clone(), &mut variables);
                    display_copy_info(&mut stdout, command);
                } else {
                    display_error(&mut stdout, String::from("There isn't a command currently selected."));
                }                  
            },                     
            Ok(Key::Ctrl('v')) => {// Paste text from the clipboard, needed for adding content to the database            
                let mut clipboard = ClipboardContext::new().unwrap();               
                if let Ok(text) = clipboard.get_contents() {
                    query.push_str(text.as_str());
                    let command = format!("{}", query);
                    write_output(&mut stdout, command); 
                } 
            },            
            Ok(Key::Char(c)) if c != '\n'  => { 
                results_selection_mode = false;               
                query.push(c);
                update_prompt(&mut stdout, &selected_command, &current_mode, &query);
                if search_mode {
                    if query.len() > 0 {
                        results.clear();
                        let command = format!("search {}", query);
                        results = search_commands(&db, &mut stdout, &command).await;
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
                clear_display(&mut stdout);
                let command_output: String = execute_command(&db, &"help".to_string()).await;
                write_output(&mut stdout, command_output);
            }
            Ok(Key::Ctrl('h')) => {// Display selectable list of past commands
                if command_history.len() > 0 {
                    clear_display(&mut stdout);
                    results.clear();
                    display_selectable_list(&mut stdout, &mut command_history);
                    results = command_history.clone();
                    results_selection_mode = false;
                    history_mode = true;
                } else {
                    display_error(&mut stdout, String::from("History is currently empty."));
                }                
            }            
            Ok(Key::Char('\n')) => {
                let command_output: String;
                history_mode = false;
                if query.starts_with("update") { //update command
                    clear_display(&mut stdout);
                    let _update_output = execute_update_command(&db, &mut stdout, &query, &mut command_history[selected_command_in_history], &mut variables).await;
                    // write_output(&mut stdout, update_output);
                    // display_command_info(&mut stdout, command_history[selected_command_in_history].clone(), &mut variables);
                } else if results_selection_mode == true { //pressed enter while arrowing through selectable list
                    let command = variables.replace_variables_in_command(&results[selected_result_index].cmd);
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(command.clone()).unwrap();
                    //Add new command to command_history if it isn't already in the command_history
                    //Also set the index
                    let mut add_to_history: bool = true;
                    let commands_slice: &[command::Command] = &command_history;
                    for (i, command) in commands_slice.iter().enumerate(){
                        if command.cmd_id == results[selected_result_index].cmd_id.to_owned(){
                            add_to_history = false;
                            selected_command_in_history = i;
                            break;
                        }
                    }
                    if add_to_history {
                        command_history.push(results[selected_result_index].clone());
                        selected_command_in_history = command_history.len() - 1;
                    }
                    clear_display(&mut stdout);
                    display_command_info(&mut stdout, results[selected_result_index].clone(), &mut variables);
                    display_copy_info(&mut stdout, command);
                    results_selection_mode = false;
                    search_mode = false;                    
                } else if query.starts_with("hist") { //history command
                    clear_display(&mut stdout);
                    if command_history.len() > 0 {
                        results.clear();
                        display_selectable_list(&mut stdout, &mut command_history);
                        results = command_history.clone();
                        results_selection_mode = false;
                    } else {
                        display_error(&mut stdout, String::from("History is currently empty."));
                    }
                } else if query.starts_with("env"){ //show user set variables
                    display_user_variables(&mut stdout, &mut variables);    
                } else if query.starts_with("set") { //set variables
                    clear_display(&mut stdout);
                    let query_values: Vec<&str> = query.split_whitespace().collect();
                    if let Some(variable) = query_values.get(1) {
                        if let Some(value) = query_values.get(2) {
                            variables.set_user_variable(variable.to_string(), value.to_string());
                        } else {
                            display_error(&mut stdout, "You must supply a value".to_string());
                        }

                        if command_history.len() > 0 {                                     
                            display_command_info(&mut stdout, command_history[selected_command_in_history].clone(), &mut variables);
                        }

                        //copy the current command to the clipboard again after setting a variable
                        let command = variables.replace_variables_in_command(&command_history[selected_command_in_history].cmd);
                        let mut clipboard = ClipboardContext::new().unwrap();
                        clipboard.set_contents(command.clone()).unwrap();
                        display_copy_info(&mut stdout, command);
                    } else {
                        display_error(&mut stdout, "You must supply a variable".to_string());
                    }
                } else if query.starts_with("info") { //info command
                    clear_display(&mut stdout);
                    if command_history.len() > 0 {
                        display_command_info(&mut stdout, command_history[selected_command_in_history].clone(), &mut variables)
                    } else {
                        display_error(&mut stdout, String::from("There isn't a command currently selected."));
                    }                            
                } else if query.starts_with("search") {
                    results = search_commands(&db, &mut stdout, &query).await;
                } else if query.starts_with("exit") {
                    break;
                } else {
                    clear_display(&mut stdout);
                    command_output = execute_command(&db, &query).await;
                    write_output(&mut stdout, command_output);
                }                
                query.clear();
            }
            _ => {}
        }
    }
}

