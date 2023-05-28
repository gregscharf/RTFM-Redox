use sqlx::SqlitePool;
use termion::raw::RawTerminal;
use std::io::Stdout;
use crate::terminal_output::display_error;
use crate::terminal_output::display_selectable_list;

pub mod command {
    use sqlx::{Row};

    #[derive(Clone)]
        pub struct Command {
        pub cmd_id: i32,
        pub cmd: String,
        pub cmnt: String,
        pub author: String,
    }

    impl Command {
        pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
            let cmd_id: i32 = row.get("CmdID");
            let cmd: String = row.get("Cmd");
            let cmnt: String = row.get("cmnt");
            let author: String = row.get("author");
            Ok(Command { cmd_id, cmd, cmnt, author })
        }
    }
}

async fn fetch_commands(db: &sqlx::SqlitePool, search_term: String) -> Result<Vec<command::Command>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT CmdID, Cmd, cmnt, author 
        FROM TblCommand 
        WHERE Cmd LIKE ? LIMIT 25",   
        ).bind(search_term).
        fetch_all(db)
        .await?;

    let commands: Result<Vec<command::Command>, sqlx::Error> = rows.iter().map(|row| command::Command::from_row(row)).collect();

    commands
}

pub async fn search_commands(db: &SqlitePool, stdout: &mut RawTerminal<Stdout>, command: &String) -> Vec<command::Command>{
        let mut results: Vec<command::Command> = Vec::new();         
        let start_index: usize = "search ".len();
        let search_term: String = format!("{}{}{}","%",&command[start_index..],"%");            

        let commands:Result<Vec<command::Command>,_>  = fetch_commands(&db, search_term).await;
        match commands {
            Ok(commands) => {
                for command in commands.iter() {
                    let new_command = command::Command {
                        cmd_id: command.cmd_id,
                        cmd: command.cmd.to_owned(),
                        cmnt: command.cmnt.to_owned(),
                        author: command.author.to_owned(),
                    };
                    results.push(new_command);
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }

        if !results.is_empty() {
            display_selectable_list(stdout, &mut results);
        } else {
            display_error(stdout, String::from("Not Found"));
        }
        return results;
}

pub async fn execute_update_command(db: &SqlitePool, command: &String, table_row: &mut command::Command) -> String {

    let content: &str;
    let column: &str;
    let sql_query: & str;
    if command.contains("comment") {
        column = "comment";
        let start_index = command.find("comment").unwrap() + "comment".len() + 1;
        content = &command[start_index..];
        sql_query = "UPDATE TblCommand SET cmnt = ? where CmdID = ?";     
        table_row.cmnt = content.to_string();  
    } else if command.contains("author"){
        column = "author";
        let start_index = command.find("author").unwrap() + "author".len() + 1;
        content = &command[start_index..];
        sql_query = "UPDATE TblCommand SET author = ? where CmdID = ?";
        table_row.author = content.to_string();

    } else if command.contains("command"){
        column = "command";
        let start_index = command.find("command").unwrap() + "command".len() + 1;
        content = &command[start_index..];
        sql_query = "UPDATE TblCommand SET Cmd = ? where CmdID = ?";
        table_row.cmd = content.to_string();
    } else {      
        return String::from("Invalid update.");
    }
    
    sqlx::query(&sql_query)
        .bind(content)
        .bind(table_row.cmd_id)
        .execute(db)
        .await
        .unwrap();

    let command_output: String = format!("Updated {}: {}\n\r",
        column,
        content);

    return command_output;

}


pub async fn execute_command(db: &SqlitePool, command: &String) -> String{
    let output;
    match command.as_str() {
        s if s.starts_with("help") => {
            if command.contains("add") {
                output = "To add a command to the database\n\r'add -c command [optional: -d comment]"; 
            } else if command.contains("search"){
                output = "Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rEsc\t\t Exit search mode.\n\rOr use 'search' command followed by a term to search results.";             
            } else {
                output = "Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rCrtl+h or hist\t Display selectable history of already selected commands.\n\rCtrl+v\t\t Paste from clipboard\n\rEsc\t\t Exit current mode.\n\rinfo\t\t Display info on the currently selected command.\n\rhelp\t\t Display help\n\radd\t\t Add a command to the database e.g. 'add -c stty raw -echo;fg'\n\rupdate\t\t update database columns of currently selected command e.g. 'update comment bash reverse shell'.\n\rCtrl+q or exit\t Exit redOx.\n\r"; 
            }
        },
        s if s.starts_with("set") => {
            output = "Attempted to set a non-existent variable.";
        }        
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
        },
        _ => {
            output = "Invalid command";
        }
    }
    return format!("{}\n\r",output);
}


