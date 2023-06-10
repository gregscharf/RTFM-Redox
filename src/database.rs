pub mod database {
    use sqlx::{migrate::MigrateDatabase,Sqlite, SqlitePool};
    use sqlx::Row;
    use sqlx::{Error as SqlxError};
    use super::command_table;

    #[derive(Clone)]
    pub struct Database {
        db: Option<SqlitePool>,
        commands: Vec<command_table::Command>
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

            Ok(Self { db, commands })
        }

        // pub async fn fetch_commands(&mut self, search_term: String) -> Result<Vec<command_table::Command>, sqlx::Error> {

        //     let rows = sqlx::query(
        //         "SELECT CmdID, Cmd, cmnt, author 
        //         FROM TblCommand 
        //         WHERE Cmd LIKE ?",   
        //         ).bind(search_term).
        //         fetch_all(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
        //         .await?;
        //     let commands: Result<Vec<command_table::Command>, sqlx::Error> = rows.iter().map(|row| command_table::Command::from_row(row)).collect();
        //     commands
        // }   

        pub async fn fetch_commands_with_references(&mut self, column: String, search_term: String) -> Result<Vec<command_table::Command>, sqlx::Error> {
            let sql_query = format!(
                "SELECT TblCommand.CmdID, TblCommand.Cmd, TblCommand.cmnt, TblCommand.author, TblRefContent.ID, TblRefContent.Ref
                FROM TblCommand
                LEFT JOIN TblRefMap ON TblCommand.CmdID = TblRefMap.CmdID
                LEFT JOIN TblRefContent ON TblRefMap.RefID = TblRefContent.ID
                WHERE {} LIKE ?", column);
            
            let rows = sqlx::query(&sql_query,)
                .bind(search_term)
                .fetch_all(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
                .await?;
        
            let mut commands: Vec<command_table::Command> = Vec::new();
        
            for row in rows {
                let cmd_id: i32 = row.try_get("CmdID")?;
                let cmd: String = row.try_get("Cmd")?;
                let cmnt: String = row.try_get("cmnt")?;
                let author: String = row.try_get("author")?;
                let ref_id: i32 = row.try_get("ID")?;
                let ref_value: String = row.try_get("Ref")?;
        
                let command = match commands.iter_mut().find(|c| c.cmd_id == cmd_id) {
                    Some(existing_command) => existing_command,
                    None => {
                        let new_command = command_table::Command {
                            cmd_id,
                            cmd: cmd.clone(),
                            cmnt: cmnt.clone(),
                            author: author.clone(),
                            references: Vec::new(),
                            tags: Vec::new(),
                        };
                        commands.push(new_command);
                        commands.last_mut().unwrap()
                    }
                };
        
                command.references.push(command_table::References { ref_id, ref_value });

            }
        
            Ok(commands)
        }

        pub async fn search_commands(&mut self, column: String, query: &String) -> Vec<command_table::Command>{     
            let start_index: usize = "search ".len();
            let search_term: String = format!("%{}%",&query[start_index..]); 

            self.commands = self.fetch_commands_with_references(column, search_term).await.unwrap();           
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
            let new_command = command_table::Command { cmd_id: row_id as i32, cmd: command.to_string(), cmnt: description.to_string(), author: String::from(""), references: Vec::new(), tags: Vec::new() };
            
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
                    } else if table_column.contains("references"){
                        return self.add_reference_to_command(command.clone(), content).await;
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
        pub async fn add_reference_to_command(&mut self, mut command: command_table::Command, reference: String) -> Result<String, sqlx::Error> {
            let ex_query = sqlx::query(
                "INSERT INTO TblRefContent (Ref) VALUES (?)",
            )
            .bind(reference.clone())
            .execute(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
            .await?;
        
            let ref_id = ex_query.last_insert_rowid();
        
            sqlx::query(
                "INSERT INTO TblRefMap (RefID, CmdID) VALUES (?, ?)",
            )
            .bind(ref_id as i32)
            .bind(command.cmd_id)
            .execute(self.db.as_ref().ok_or(SqlxError::RowNotFound)?)
            .await?;        

            command.references.push(command_table::References { ref_id: ref_id as i32, ref_value: reference.clone()});

            let command_output: String = format!("Added {} to references\n\r",
                reference);

            Ok(command_output)  
        }        

    }
}

pub mod command_table {
    use crate::command_variables;

    // use sqlx::Row;
    #[derive(Clone)]
    pub struct Command {
        pub cmd_id: i32,
        pub cmd: String,
        pub cmnt: String,
        pub author: String,
        pub references: Vec<References>,
        pub tags: Vec<Tags>,
    }
    
    #[derive(Clone)]
    pub struct References {
        pub ref_id: i32,
        pub ref_value: String,
    }

    #[derive(Clone)]
    pub struct Tags {
        pub tag_id: i32,
        pub tag_value: String,
    }

    impl Command {
        pub fn url_encode(&mut self, command_variables: command_variables::variables::Variables) -> String {
            let mut encoded = String::new(); 
            
            let command_syntax = command_variables.clone().replace_variables_in_command(&self.cmd);

            for byte in command_syntax.bytes() {
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

            encoded
        }

        pub fn set_command_variables (&mut self, command_variables: command_variables::variables::Variables) -> String {
            command_variables.clone().replace_variables_in_command(&self.cmd)
        }

    }
}


