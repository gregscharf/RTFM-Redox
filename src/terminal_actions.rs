use crate::database::command_table;
use crate::database::database;
use crate::terminal_output;


pub async fn execute_search(search_term: String, database: &mut database::Database) -> Vec<command_table::Command>{
    let terminal_output = &mut terminal_output::output::Output::new();
    let mut command_results: Vec<command_table::Command>;

    command_results = database.search_commands(&search_term).await; 
       
    if !command_results.is_empty() {
        terminal_output.display_selectable_list(&mut command_results);
    } else {
        terminal_output.display_error(format!("No results found for {}", search_term));
    }   

    command_results
}


pub async fn execute_help(command: String) {
    let terminal_output = &mut terminal_output::output::Output::new();
    terminal_output.clear_display();

    let min_width: usize = 10;
    let output: String;

    //All help commands and messages
    let control_r:String = format!("{:<width$}Enter quick search mode to dynamically find commands as you type.",String::from("Ctrl+r"),width = min_width);
    let control_c: String = format!("{:<width$}Copy currently selected command to clipboard.",String::from("Ctrl+c"),width = min_width);
    let control_u: String = format!("{:<width$}URL-encode and then copy currently selected command to clipboard.",String::from("Ctrl+u"),width = min_width);
    let control_h: String = format!("{:<width$}\n\r{:<width$}Display selectable history of already selected commands.",String::from("Crtl+h"),String::from("or 'hist'"),width = min_width);
    let control_v: String = format!("{:<width$}Paste from clipboard",String::from("Crtl+v"),width = min_width);
    let info: String = format!("{:<width$}Display info on the currently selected command.",String::from("info"),width = min_width);
    let env: String = format!("{:<width$}Show user variables that have already been set.",String::from("env"),width = min_width);
    let add: String = format!("{:<width$}Add a command to the database e.g. 'add -c stty raw -echo;fg'",String::from("add"),width = min_width);
    let update: String = format!("{:<width$}Update a database column in the selected command\n\r{:<width$}e.g. comment, command, author or references\n\r{:<width$}Example: update references http://blog.gregscharf.com",String::from("update"),String::from(" "),String::from(" "), width = min_width);
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
        output = format!("{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r{}\n\r",
            control_r,
            control_c,
            control_u,
            control_h,
            control_v,
            info,
            env,
            add,
            update,
            esc,
            help,
            exit); 
    }  

    terminal_output.write_output(output);

}
