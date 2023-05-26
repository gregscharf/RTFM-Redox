use sqlx::{SqlitePool,Row};
use termion::{color};
use termion::raw::{RawTerminal};
use std::io::Stdout;
use crate::console_view::display_error;
use crate::console_view::display_selectable_list;

// use crate::Command;


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

pub async fn execute_command(db: &SqlitePool, command: &String) -> String{
    let output;
    match command.as_str() {
        s if s.starts_with("open") => {
            output = "Open";
        }
        "info" => {
            output = "Info";
        }
        s if s.starts_with("use") => {
            output = "Use";
        }                
        s if s.starts_with("help") => {
            if command.contains("add") {
                output = "To add a command to the database\n\r'add -c command [optional: -d comment]"; 
            } else if command.contains("search"){
                output = "Ctrl+r -- to enter quick search mode to find matching commands as you type.\n\rEsc to exit search mode.\n\rOr use 'search' command followed by a term to search results.";             
            } else {
                output = "exit or Ctrl+c\t-- Exit redOx.\n\rCtrl+v\t\t-- Paste from clipboard\n\rCtrl+r\t\t-- Enter search mode and then start typing to find commands.\n\rEsc\t\t-- Exit current mode.\n\rhistory\t\t-- Display selectable history of already selected commands\n\rinfo\t\t-- Display info on currently selected command.\n\r"; 
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


