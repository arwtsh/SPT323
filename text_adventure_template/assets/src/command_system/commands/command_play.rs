use crate::command_system::commands::Command;

/// Immediately exits the application.
pub struct CommandPlay;

impl Command for CommandPlay {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "play".to_string(),
            "PLAY".to_string(),
            "Play".to_string(),
            "Start".to_string(),
            "start".to_string(),
            "START".to_string(),
            "p".to_string(),
            "P".to_string(),
            "s".to_string(),
            "S".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {

    }
}