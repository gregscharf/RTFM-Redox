use sqlx::{SqlitePool,Row};
use termion::{color};

pub async fn execute_search_command(db: &SqlitePool, command: &String) -> (String, Vec<String>){
        let mut results: Vec<String> = Vec::new();            
        let start_index: usize = "search".len() + 1;
        let search_term: String = format!("{}{}{}","%",&command[start_index..],"%");            
        let rows = sqlx::query(
            "SELECT CmdID, Cmd, cmnt 
                FROM TblCommand 
                WHERE Cmd LIKE ? LIMIT 25",
            )
            .bind(search_term)
            .fetch_all(db)
            .await
            .unwrap();

        for (_idx, row) in rows.iter().enumerate() {
            results.push(row.get::<String, &str>("Cmd"));
        }
        let mut query_output = String::new();

        if !results.is_empty() {
            query_output = format!("{}{}{}{}{}",
            color::Bg(color::White),
            color::Fg(color::Black),
            "Select a result with the up/down arrow keys. Press enter to copy to the clipboard\n\r",
            color::Fg(color::Reset),
            color::Bg(color::Reset));            
            for (i, command) in results.iter().enumerate() {
                query_output += format!("({}) - {}\n\r",i,command).as_str();
            }

        } else {
            query_output = format!("{}{}{}",
                    color::Fg(color::Red),
                    "Not Found\n\r",
                    color::Fg(color::Reset));
        }
        return (query_output,results);
}

pub async fn execute_command(db: &SqlitePool, command: &String) -> String{
    let output;
    match command.as_str() {
        "open" => {
            // let file_path = Select::new()
            // .with_prompt("Select a file")
            // .show_hidden(true)
            // .can_select_directories(true)
            // .default("my_file.txt")
            // .interact()?;
            // let command_output: String = format!("You selected file: {}\n\r",file_path.display());
            // return command_output;
            output = "Open";
        }
        s if s.starts_with("use") => {
            output = "Use";
        }                
        s if s.starts_with("help") => {
            if command.contains("add") {
                output = "To add a command to the database\n\r'add -c command [optional: -d comment"; 
            } else if command.contains("search"){
                output = "ctrl+r to enter quick search mode to find matching commands as you type.\n\rEsc to exit search mode.\n\rOr use 'search' command followed by a term to search results.";             
            } else {
                output = "ctrl+c to exit.\n\rctrl+v to paste from clipboard\n\rctrl+r to enter search mode to find syntax.\n\rEsc to exit search mode."; 
            }
        },
        s if s.starts_with("add") => {
            if command.contains(" -c "){
                let start_index = command.find("-c").unwrap() + 3;
                let mut end_index = command.len();
                let mut description = "None";
                if command.contains(" -d ") {
                    let start_desc_index = command.find("-d").unwrap() + 3;
                    description = &command[start_desc_index..end_index];
                    end_index = command.find("-d").unwrap() - 1;
                }
                let command = &command[start_index..end_index];

                sqlx::query(
                    "INSERT INTO TblCommand (Cmd, cmnt) VALUES (?, ?)",
                    ).bind(command)
                    .bind(description)
                    .execute(db)
                    .await
                    .unwrap();

                let command_output: String = format!("Inserted command: {} comment: {}\n\r",command,description);
                return command_output;
            } else {
                output = "To add a command to the database you must include -c followed by the command\n\r-d with a description of the command is optional"; 
            }

            // output = "set add"; 
        },
        _ => {
            output = "Invalid command";
        }
    }
    let command_output: String = format!("{}\n\r",output);    
    return command_output;
}


