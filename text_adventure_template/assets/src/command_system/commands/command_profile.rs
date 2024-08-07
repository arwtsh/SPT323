use crate::command_system::commands::Command;

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandProfile;

impl Command for CommandProfile {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "profile".to_string(),
            "Profile".to_string(),
            "PROFILE".to_string(),
            "Pr".to_string(),
            "PR".to_string(),
            "pr".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {

    }
}