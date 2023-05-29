use sqlx::SqlitePool;
use termion::raw::RawTerminal;
use std::io::Stdout;
use crate::command_variables;
use crate::terminal_output::{display_error, display_command_info, display_selectable_list, write_output};

pub mod command {
    use sqlx::Row;

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

pub async fn execute_update_command(db: &SqlitePool, stdout: &mut RawTerminal<Stdout>, query: &String, command: &mut command::Command, variables: &mut command_variables::variables::Variables) -> bool {
    let command_values: Vec<&str> = query.split_whitespace().collect();
    if let Some(table_column) = command_values.get(1) {
        if command_values.len() > 2 {
            let sql_query: & str;
            let content = command_values[2..].join(" ");
            if table_column.contains("comment") {
                sql_query = "UPDATE TblCommand SET cmnt = ? where CmdID = ?";     
                command.cmnt = content.to_string();  
            } else if table_column.contains("author"){
                sql_query = "UPDATE TblCommand SET author = ? where CmdID = ?";
                command.author = content.to_string();
        
            } else if table_column.contains("command"){
                sql_query = "UPDATE TblCommand SET Cmd = ? where CmdID = ?";
                command.cmd = content.to_string();
            } else {   
                display_error(stdout, "Update failed.".to_string());   
                return false;
            }

            sqlx::query(&sql_query)
                .bind(content.clone())
                .bind(command.cmd_id)
                .execute(db)
                .await
                .unwrap();
           
            let command_output: String = format!("Updated {}: {}\n\r",
                table_column,
                content);
                display_command_info(stdout, command.clone(), variables);
                write_output(stdout, command_output);
            return true;
        } else {
            let error: String = format!("You must supply a value for column {}.\n\rExample: update {} content to add",
                table_column,
                table_column);           
            display_error(stdout, error);
            return false;
        }
    } else {
        display_error(stdout, "You must supply a column name. See the currently selected commands's 'info'".to_string());
        return false;
    }
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
                output = "Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rCtrl+c\t\t Copy currently selected command to clipboard.\n\rCtrl+u\t\t Url-encode and then copy currently selected command to clipboard.\n\rCrtl+h or hist\t Display selectable history of already selected commands.\n\rCtrl+v\t\t Paste from clipboard\n\rinfo\t\t Display info on the currently selected command.\n\renv\t\t Show currently set user variables\n\radd\t\t Add a command to the database e.g. 'add -c stty raw -echo;fg'\n\rupdate\t\t update database columns of currently selected command e.g. 'update comment bash reverse shell'.\n\rEsc\t\t Exit current mode.\n\rhelp\t\t Display help\n\rCtrl+q or exit\t Exit redOx.\n\r"; 
            }
        },     
        s if s.starts_with("add") => {
            if command.contains(" -c "){
                let start_index = command.find("-c").unwrap() + 3;
                //TODO: check if there is a -d after and if so set that as the ending index for our command
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

                let command_output: String = format!("Inserted command: {} comment: {}\n\r",command, description);
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


