use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType::MoveLeft;

use crate::command_system::commands::Command;

/// A way for the player to move locations.
/// Moves the player to the scene marked "left"
pub struct CommandLeft;

impl Command for CommandLeft {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "left".to_string(),
            "LEFT".to_string(),
            "Left".to_string(),
            "l".to_string(),
            "L".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        get_event_system().invoke(MoveLeft);
    }
}