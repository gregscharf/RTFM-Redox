use crate::database::command_table;
use crate::database::database;
use crate::terminal_output;

pub async fn execute_search(search_column: String, search_term: String, database: &mut database::Database) -> Option<Vec<command_table::Command>>{
    let terminal_output = &mut terminal_output::output::Output::new();

    let command_values: Vec<&str> = search_term.split_whitespace().collect();
    if command_values.len() < 2 {
        terminal_output.display_error("No search term entered".to_string());
        return None;
    }
    
    let mut command_results: Vec<command_table::Command>;
    command_results = database.search_commands(search_column,&search_term).await; 
       
    if !command_results.is_empty() {
        terminal_output.display_selectable_list(&mut command_results);
    } else {
        terminal_output.display_error(format!("No results found for {}", search_term));
        return None;
    }   

    Some(command_results)
}

pub async fn execute_help(command: String) {
    let terminal_output = &mut terminal_output::output::Output::new();
    terminal_output.clear_display();

    let min_width: usize = 10;
    let output: String;

    //All help commands and messages
    let control_r:String = format!("{:<width$}Activates a dynamic search for a command or a command comment as you type.\n\r{:<width$}Click Ctrl+r repeatedly to cycle between command search or comment search.",String::from("Ctrl+r"),String::from(" "),width = min_width);
    let control_c: String = format!("{:<width$}Copy the selected command to the clipboard.",String::from("Ctrl+c"),width = min_width);
    let control_u: String = format!("{:<width$}URL-encode and then copy the selected command to the clipboard.",String::from("Ctrl+u"),width = min_width);
    let control_h: String = format!("{:<width$}Display selectable history of already selected commands.",String::from("Crtl+h"),width = min_width);
    let hist: String = format!("{:<width$}Same as Crtl+h",String::from("hist"),width = min_width);
    let control_v: String = format!("{:<width$}Paste from the clipboard. Note: Your terminal's native paste key combination will not work.",String::from("Crtl+v"),width = min_width);
    let info: String = format!("{:<width$}Display info on the selected command.",String::from("info"),width = min_width);
    let env: String = format!("{:<width$}Show user variables that have already been set.",String::from("env"),width = min_width);
    let set: String = format!("{:width$}Set a user variable Example: set lhost 10.10.16.3\n\r{:<width$}Variables are case insensitive",String::from("set"),String::from(" "),width = min_width);
    let add: String = format!("{:<width$}Add a command to the database Example: add -c stty raw -echo;fg",String::from("add"),width = min_width);
    let update: String = format!("{:<width$}Update a database column (i.e. comment, command, author, and references) in the selected command\n\r{:<width$}.\n\r{:<width$}Example: update references https://blog.gregscharf.com",String::from("update"),String::from(" "),String::from(" "), width = min_width);
    let esc: String = format!("{:<width$}Exit current mode",String::from("Esc"),width = min_width);
    let help: String = format!("{:<width$}Display help",String::from("help"),width = min_width);
    let exit: String = format!("{:<width$}Exit RedOx",String::from("Ctrl+q"),width = min_width);
    
    //Detailed help messages
    let add_detail: String = format!("To add a command to the database\n\r'add -c command [optional: -d comment]");
    let search_detail: String = format!("{}\n\r{}\n\rOr use 'search' command followed by a term to search results.",control_r,esc);
    
    if command.contains("add") {
        output = add_detail; 
    } else if command.contains("search"){
        output = search_detail;             
    } else {
        output = format!("{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r",
            control_r,
            control_c,
            control_u,
            control_h,
            hist,
            control_v,
            info,
            env,
            set,
            add,
            update,
            esc,
            help,
            exit); 
    }  

    terminal_output.write_output(output);

}
