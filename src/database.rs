pub mod database {
    use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
    use sqlx::{Error as SqlxError};
    use super::command_table;

    #[derive(Clone)]
    pub struct Database {
        pub db_url: String,
        pub db: Option<SqlitePool>,
        pub commands: Vec<command_table::Command>
    }

    impl Database {
        pub async fn new() -> Result<Self, sqlx::Error> {
            let db_url = "sqlite://snips.db".to_string();

            // Check if database exists
            if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
                return Err(sqlx::Error::RowNotFound);
            }

            let db = match SqlitePool::connect(&db_url).await {
                Ok(db) => Some(db),
                Err(error) => {
                    return Err(SqlxError::from(error));
                }
            };            
            let commands: Vec<command_table::Command> = Vec::new();
            Ok(Self { db_url, db, commands })

        }

        pub async fn fetch_commands(&mut self, search_term: String) -> Result<Vec<command_table::Command>, sqlx::Error> {

            let rows = sqlx::query(
                "SELECT CmdID, Cmd, cmnt, author 
                FROM TblCommand 
                WHERE Cmd LIKE ? LIMIT 25",   
                ).bind(search_term).
                fetch_all(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
                .await?;
            let commands: Result<Vec<command_table::Command>, sqlx::Error> = rows.iter().map(|row| command_table::Command::from_row(row)).collect();
            commands
        }   

        pub async fn search_commands(&mut self, query: &String) -> Vec<command_table::Command>{     
            let start_index: usize = "search ".len();
            let search_term: String = format!("{}{}{}","%",&query[start_index..],"%");            
            self.commands =  self.fetch_commands(search_term).await.unwrap();
            
            self.commands.clone()
        }

        pub async fn add_command(&mut self, query: &String) -> Result<Vec<command_table::Command>, sqlx::Error>{
            let start_index = query.find("-c").unwrap() + 3;
            //TODO: check if there is a -d after and if so set that as the ending index for our command
            let mut end_index = query.len();
            let mut description = "None";
            if query.contains(" -d ") {
                let start_desc_index: usize = query.find("-d").unwrap() + 3;
                description = &query[start_desc_index..end_index];
                end_index = query.find("-d").unwrap() - 1;
            }
            let command = &query[start_index..end_index];

            let ex_query = sqlx::query(
                "INSERT INTO TblCommand (Cmd, cmnt) VALUES (?, ?)",
                ).bind(command)
                .bind(description)
                .execute(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
                .await
                .unwrap();

            let row_id = ex_query.last_insert_rowid();
            let new_command = command_table::Command { cmd_id: row_id as i32, cmd: command.to_string(), cmnt: description.to_string(), author: String::from("") };
            self.commands.push(new_command.clone());           
            Ok(self.commands.clone())      
        }

        pub async fn update_command(&mut self, query: &String, command: &mut command_table::Command) -> Result<String, sqlx::Error> {
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
                        // display_error( "Update failed.".to_string());   
                        return Err(sqlx::Error::RowNotFound);
                    }
        
                    sqlx::query(&sql_query)
                        .bind(content.clone())
                        .bind(command.cmd_id)
                        .execute(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
                        .await
                        .unwrap();
                    
                    let command_output: String = format!("Updated {}: {}\n\r",
                        table_column,
                        content);
                        return Ok(command_output);
                    // return true;
                } else {
                    // let error: String = format!("You must supply a value for column {}.\n\rExample: update {} content to add",
                    //     table_column,
                    //     table_column);           
                    // display_error(error);
                    return Err(sqlx::Error::RowNotFound);
                }
            } else {
                // display_error("You must supply a column name. See the currently selected commands's 'info'".to_string());
                return Err(sqlx::Error::RowNotFound);
            }

        }        
    }
}



pub mod command_table {
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

