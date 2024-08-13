use crate::command_system::commands::Command;

/// Immediately exits the application.
pub struct CommandCredits;

impl Command for CommandCredits {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "credits".to_string(),
            "CREDITS".to_string(),
            "Credits".to_string(),
            "c".to_string(),
            "C".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        println!("Made by Adam White.");
    }
}