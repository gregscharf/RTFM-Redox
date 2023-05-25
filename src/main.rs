mod console_view;
use console_view::{highlight_search_result,write_output,update_prompt};
mod execute_command; 
use execute_command::{execute_command,execute_search_command};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::io::{Write, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use termion::{color, cursor, terminal_size};
use clipboard::{ClipboardContext, ClipboardProvider};
// use x11_clipboard::{Clipboard};

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
    let mut results: Vec<String> = Vec::new();
    let mut search_mode: bool = false;
    
    // Set up the scrolling output buffer
    // let mut output: Vec<String> = vec![];
    // let mut total_output: i32 = 1;
    // let mut output_start = 0;

    let mut selected_result_index: usize = 0;
    let mut results_selection_mode: bool = false;

    let (_width, height) = terminal_size().unwrap();

    let command_output: String = execute_command(&db, &"help".to_string()).await;
    write_output(&mut stdout, command_output);
  
    loop {
        let mut prompt = "redOx:";  
        if search_mode { 
            prompt = "redOx(find):";
        }
        update_prompt(&mut stdout, prompt, &query);

        let key = std::io::stdin().keys().next().unwrap();

        match key {          
            Ok(Key::Ctrl('r')) => { // Ctrl + R to enter search mode and query the database as you type
                search_mode = true;
                query.clear();
            }
            Ok(Key::Backspace) => {
                query.pop();
                write!(stdout, "{}{}", 
                    termion::cursor::Left(prompt.len() as u16 + query.len() as u16 + 1), 
                    termion::clear::AfterCursor)
                    .expect("Failed to write to stdout");
                    if search_mode  {
                        results.clear(); 
                        results_selection_mode = false;                      
                        let command_output: String;
                        
                        if query.len() > 0 { 
                            let command = format!("search {}", query);
                            (command_output, results) = execute_search_command(&db, &command).await;
                        } else {
                            command_output = String::new();
                        }
                        write_output(&mut stdout, command_output);                           
                    } 
            }
            Ok(Key::Up) => {  // Move up in results  
                if results_selection_mode == false && results.len() > 0{
                    results_selection_mode = true;
                    selected_result_index = results.len();
                }             
                if results.len() > 0 {                   
                    if selected_result_index > 0 {
                        selected_result_index -= 1;
                        highlight_search_result(&mut stdout,selected_result_index, &mut results);    
                    }
                }
            }
            Ok(Key::Down) => {// Move down in results             
                if results.len() > 0 {
                    if results_selection_mode == false && results.len() > 0{
                        results_selection_mode = true;
                        selected_result_index = 0;
                    }  

                    results_selection_mode = true;
                    if selected_result_index < results.len() - 1 {
                        selected_result_index += 1;
                        highlight_search_result(&mut stdout,selected_result_index, &mut results); 
                    }
                }
            }            
            Ok(Key::Ctrl('v')) => {// Paste text from the clipboard            
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
                write!(stdout, "{}", 
                    c)
                    .expect("Failed to write to stdout");
                    if search_mode {
                        if query.len() > 0 {
                            results.clear();
                            let mut command_output = String::new();
                            let command = format!("search {}", query);
                            (command_output, results) = execute_search_command(&db, &command).await;
                            write_output(&mut stdout, command_output);                           
                        }
                    }
            }            
            Ok(Key::Ctrl('c')) => {// Exit the CLI
                break
            }
            Ok(Key::Esc)  => {
                search_mode = false;
                results_selection_mode = false;
                query.clear();
                write!(stdout, "{}", 
                termion::clear::All)
                .expect("Failed to write to stdout");                
            }
            Ok(Key::Char('\n')) => {
                let mut command_output: String = String::new();
                if results_selection_mode == true {
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(results[selected_result_index].to_owned());
                    command_output = format!("Copied: {} to clipboard\n\r",results[selected_result_index]);
                    write_output(&mut stdout, command_output);
                    results_selection_mode == false;
                } else if query.starts_with("search") {
                    (command_output,results) = execute_search_command(&db, &query).await;
                    write_output(&mut stdout, command_output);
                } else {
                    command_output = execute_command(&db, &query).await;
                    write_output(&mut stdout, command_output);
                }                
                query.clear();
            }
            _ => {}
        }
    }
}

