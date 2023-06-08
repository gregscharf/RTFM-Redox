pub mod search {
    use crate::database::command_table;
    use crate::terminal_output;

    pub(crate) const OFF: i32 = -1;
    //Various search modes to cycle through with Crtl+r
    pub(crate) const COMMAND_SEARCH: i32 = 0;
    pub(crate) const COMMENT_SEARCH: i32 = 1;
    pub(crate) const TAG_SEARCH: i32 = 2;
    pub(crate) const SEARCH_MODES: [&str; 2] = ["command","comment"];
    pub(crate) const SEARCH_COLUMNS: [&str; 2] = ["Cmd","cmnt"];
    pub(crate) const UP: i32 = 1;
    pub(crate) const DOWN: i32 = -1;

    #[derive(Clone)]
    pub struct Results {
        pub results: Vec<command_table::Command>,
        pub history: Vec<command_table::Command>,
        pub current_selection: usize, 
        pub search_mode: i32,
        pub history_mode: bool,
        pub results_selection_mode: bool,
        pub selected_result_index: i32,
        pub selected_history_index: i32,
        pub search_modes: Vec<i32>,
    }

    impl Results {
        pub fn new() -> Self {
            //TODO: Save/Load user variables from config file
            Self {
                //Create a history for selected commands
                history: Vec::new(),
                results: Vec::new(),
                current_selection: 0,
                search_mode: -1,
                history_mode: false,
                selected_result_index: -1,
                selected_history_index: -1,
                results_selection_mode: false,
                search_modes: vec![COMMAND_SEARCH,COMMENT_SEARCH,TAG_SEARCH],
            }
        }

        pub fn get_selected_history_command(&mut self) -> Result<command_table::Command, Box<dyn std::error::Error>> {
            if self.selected_history_index == -1 {
                Err("History is empty.".into())
            } else {
                let command = self.history[self.selected_history_index as usize].clone();
                Ok(command)
            }
        }

        pub fn reset(&mut self) {
            self.results.clear();
            self.search_mode = OFF;
            self.results_selection_mode = false;
            self.history_mode = false;
            self.selected_result_index = OFF;
            // self.selected_history_index = OFF;
        }

        pub fn set_results(&mut self, current_results: Vec<command_table::Command>) {
            self.results.clear();
            self.results = current_results;
            self.results_selection_mode = false;
            self.selected_result_index = self.results.len() as i32 - 1;
        }

        pub fn set_history_mode(&mut self, state: bool) {
            self.history_mode = state;
            if state {
                self.results_selection_mode = false;
            }
        }

        pub fn set_search_mode(&mut self, state: i32) {
            if state == OFF {
                self.search_mode = OFF;
                self.results_selection_mode = false;
            } else {
                self.search_mode = COMMAND_SEARCH;             
            }
        }

        pub fn get_search_mode(&mut self) -> i32 {
            self.search_mode
        }
        
        pub fn get_search_column(&mut self) -> String {
            SEARCH_COLUMNS[self.search_mode as usize].to_string()
        }

        pub fn get_results_selection_mode(&mut self) -> bool {
            self.results_selection_mode
        }

        pub fn get_history(&mut self) -> Vec<command_table::Command>{
            self.history.clone()
        }

        pub fn get_current_command_id (&mut self) -> Option<i32> {
            if !self.history.is_empty() {
                Some(self.history[self.selected_history_index as usize].cmd_id)
            } else {
                None
            }            
        }

        pub fn get_current_command_syntax (&mut self) -> Option<String> {
            if !self.history.is_empty() {
                Some(self.history[self.selected_history_index as usize].cmd.clone())
            } else {
                None
            }            
        }

        pub fn get_current_command(&mut self) -> command_table::Command{
            self.results[self.selected_result_index as usize].clone()
        }

        pub fn get_current_history_command(&mut self) -> Option<command_table::Command> {
            if !self.history.is_empty() {
                Some(self.history[self.selected_history_index as usize].clone())
            } else {
                None
            }
        }
        
        pub fn get_current_mode(&mut self) -> String {
            let mut current_mode: String = String::from("");

            if self.search_mode != -1 { 
                current_mode = format!("find[{}]",SEARCH_MODES[self.search_mode as usize]);
            } else if self.history_mode {
                current_mode = "history".to_string();
            }            

            current_mode
        }

        pub fn highlight_current_selection(&mut self, terminal_output: &mut terminal_output::output::Output) {
            terminal_output.highlight_search_result(self.selected_result_index as usize, self.results.clone()); 
        }

        //To cycle the search mode when repeatedly pressing Ctrl+r
        pub fn cycle_search_mode(&mut self) {
            self.search_mode += 1;
            let search_mode_index = self.search_mode % (self.search_modes.len() as i32 - 1);
            self.search_mode = self.search_modes[search_mode_index as usize];
        }

        pub fn cycle_through_results(&mut self, direction: i32) -> Result<(), Box<dyn std::error::Error>> {
            if self.results.len() > 0 {
                if self.results_selection_mode  {
                    self.selected_result_index = (self.selected_result_index + self.results.len() as i32 - direction) % self.results.len() as i32;
                    return Ok(());
                } else {             
                    self.results_selection_mode = true;
                    self.selected_result_index = self.results.len() as i32 - 1; //results displayed begin with highest index at bottom of screen 
                    return Ok(());               
                }
            }
            Err("Search results are empty.".into())
        }

        pub fn add_command_to_history(&mut self, command: command_table::Command){
            let mut add_to_history: bool = true;
                                
            let commands_slice: &[command_table::Command] = &self.history;
            for (i, command) in commands_slice.iter().enumerate(){
                if command.cmd_id == self.results[self.selected_result_index as usize].cmd_id.to_owned(){
                    add_to_history = false;
                    self.selected_history_index = i as i32;
                    break;
                }
            }
            
            if add_to_history {
                self.history.push(command);
                self.selected_history_index = self.history.len() as i32 - 1;
            } else { //copy any updated values
                self.history[self.selected_history_index as usize].cmd = command.cmd;
                self.history[self.selected_history_index as usize].cmnt = command.cmnt;
                self.history[self.selected_history_index as usize].author = command.author;
                self.history[self.selected_history_index as usize].references = command.references;

            }            
        }
    }
}

