pub mod variables {
    use regex::Regex;
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

        pub fn get_printable_variable_list(&mut self, variables: HashMap<String,String>) -> String{
            let mut output = String::new();

            for (key, value) in variables {
                output.push_str(&format!("{:<width$}: {}\n\r", key, if value.is_empty() { "(empty)" } else { &value },width=20));
            }

            output            
        }

        pub fn set_user_variable(&mut self, key: String, value: String) {
            self.user_variables.insert(key.clone().to_uppercase(),value.clone());
            self.command_variables.entry(key).or_insert(value);
        }

        pub fn replace_variables_in_command(&mut self, command: &str) -> String {
            let re = Regex::new(r"(?i)\\?\[([^\[\]]+)\]").unwrap();

            let replaced = re.replace_all(command, |caps: &regex::Captures<'_>| {
                if let Some(key) = caps.get(1) {
                    let key_str = key.as_str().to_lowercase();

                    if let Some(value) = self.user_variables
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(&key_str))
                        .map(|(_, v)| v)
                    {
                        return value.to_string();
                    }
                }
                caps[0].to_string()
            });

            replaced.into_owned()
        }

        pub fn extract_variables_from_command(&mut self, command: &str) -> String {
            let re = Regex::new(r"\[([^\[\]0-9]+)\]").unwrap();

            self.command_variables.clear();

            for capture in re.captures_iter(command) {
                if let Some(value) = capture.get(1) {
                    let key = value.as_str().to_owned();
                    let mut default_value = String::new();
                    if let Some(value) = self.user_variables
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(&key))
                        .map(|(_, v)| v)
                    {
                        default_value = value.to_string();
                    }
                    self.command_variables.insert(key, default_value);
                }
            }
            
            let printable_list = self.get_printable_variable_list(self.command_variables.clone());
            printable_list
        }

    }
}


