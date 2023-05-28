pub mod variables {
    use regex::Regex;
    use regex::RegexBuilder;
    use std::collections::HashMap;
    
    #[derive(Clone)]
    pub struct Variables {
        pub user_variables: HashMap<String, String>,
        pub command_variables: HashMap<String, String>,
    }

    impl Variables {
    
        pub fn new() -> Self {
            // Load user variables from config file
            Self {
                user_variables: HashMap::new(),
                command_variables: HashMap::new(),
            }
        }
        
        pub fn printable_variable_list(&mut self, variables: HashMap<String,String>) -> String{
            let mut output = String::new();
            for (key, value) in variables {
                output.push_str(&format!("{}     \t: {}\n\r", key, if value.is_empty() { "(empty)" } else { &value }));
            }
            output            
        }

        pub fn set_user_variable(&mut self, key: String, value: String) {
            self.user_variables.insert(key.clone().to_uppercase(),value.clone());
            self.command_variables.entry(key).or_insert(value);
        }

        pub fn replace_variables_in_command(&mut self, command: &str) -> String {
            let re = RegexBuilder::new(r"(?i)\\?\[([^\[\]]+)\]").case_insensitive(true).build().unwrap();
            let replaced = re.replace_all(command, |caps: &regex::Captures<'_>| {
                if let Some(key) = caps.get(1) {
                    let key_str = key.as_str();
                    if let Some(value) = self.user_variables.get(key_str) {
                        return value.to_string();
                    }
                }
                caps[0].to_string()
            });
            replaced.into_owned()
        }

        pub fn extract_variables_from_command(&mut self, command: &String) -> String {
            //exclude digits since there are commands that have syntax with [] in them
            let re = Regex::new(r"\[([^\[\]0-9]+)\]").unwrap();
            self.command_variables.clear();
            for capture in re.captures_iter(command) {
                if let Some(value) = capture.get(1) {
                    let key = value.as_str().to_owned().to_uppercase();
                    let mut default_value = String::new();
                    // If there is a corresponding user variable already set
                    if let Some(value) = self.user_variables.get(&key) {
                        default_value = value.to_string();
                    } 
                    self.command_variables.insert(key,default_value);
                }
            }
            let printable_list = self.printable_variable_list(self.command_variables.clone());
            
            printable_list
        }
    }
}


