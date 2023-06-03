pub async fn execute_command(command: &String) -> Result<String, String> {
    let output: String;
    match command.as_str() {
        s if s.starts_with("help") => {
            let min_width: usize = 17;

            //All help commands and messages
            let control_r:String = format!("{:<width$}Enter quick search mode to dynamically find commands as you type.",String::from("Ctrl+r"),width = min_width);
            let control_c: String = format!("{:<width$}Copy currently selected command to clipboard.",String::from("Ctrl+c"),width = min_width);
            let control_u: String = format!("{:<width$}URL-encode and then copy currently selected command to clipboard.",String::from("Ctrl+u"),width = min_width);
            let control_h: String = format!("{:<width$}Display selectable history of already selected commands.",String::from("Crtl+h or hist"),width = min_width);
            let control_v: String = format!("{:<width$}Paste from clipboard",String::from("Crtl+v"),width = min_width);
            let info: String = format!("{:<width$}Display info on the currently selected command.",String::from("info"),width = min_width);
            let env: String = format!("{:<width$}Show currently set user variables",String::from("env"),width = min_width);
            let add: String = format!("{:<width$}Add a command to the database e.g. 'add -c stty raw -echo;fg'",String::from("add"),width = min_width);
            let update: String = format!("{:<width$}update database columns of currently selected command\n\r{:<width$}Use the name to the left of the content e.g. update author\n\r{:<width$}Ex: update references http://blog.gregscharf.com",String::from("update"),String::from(" "),String::from(" "), width = min_width);
            let esc: String = format!("{:<width$}Exit current mode",String::from("Esc"),width = min_width);
            let help: String = format!("{:<width$}Display help",String::from("help"),width = min_width);
            let exit: String = format!("{:<width$}Exit RedOx",String::from("Ctrl+q or exit"),width = min_width);
            
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
        }
        _ => {
            return Err("Invalid command".to_string())
        }
    }
    return Ok(format!("{}\n\r",output));
}
