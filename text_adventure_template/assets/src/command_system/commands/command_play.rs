use crate::command_system::commands::Command;

/// Immediately exits the application.
pub struct CommandPlay;

impl Command for CommandPlay {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "exit".to_string(),
            "EXIT".to_string(),
            "Exit".to_string(),
            "Quit".to_string(),
            "quit".to_string(),
            "QUIT".to_string(),
            "q".to_string(),
            "Q".to_string(),
            "e".to_string(),
            "E".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {

    }
}