
pub async fn execute_command(command: &String) -> Result<String, String> {
    let output;
    match command.as_str() {
        s if s.starts_with("help") => {
            if command.contains("add") {
                output = "To add a command to the database\n\r'add -c command [optional: -d comment]"; 
            } else if command.contains("search"){
                output = "Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rEsc\t\t Exit search mode.\n\rOr use 'search' command followed by a term to search results.";             
            } else {
                output = "Ctrl+r\t\t Enter quick search mode to dynamically find commands as you type.\n\rCtrl+c\t\t Copy currently selected command to clipboard.\n\rCtrl+u\t\t URL-encode and then copy currently selected command to clipboard.\n\rCrtl+h or hist\t Display selectable history of already selected commands.\n\rCtrl+v\t\t Paste from clipboard\n\rinfo\t\t Display info on the currently selected command.\n\renv\t\t Show currently set user variables\n\radd\t\t Add a command to the database e.g. 'add -c stty raw -echo;fg'\n\rupdate\t\t update database columns of currently selected command e.g. 'update comment bash reverse shell'.\n\rEsc\t\t Exit current mode.\n\rhelp\t\t Display help\n\rCtrl+q or exit\t Exit redOx.\n\r"; 
            }
        }
        _ => {
            return Err("Invalid command".to_string())
        }
    }
    return Ok(format!("{}\n\r",output));
}
