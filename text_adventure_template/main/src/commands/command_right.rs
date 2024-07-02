use crate::scene_manager::get_mut_scene_manager;
use crate::command_manager::Command;

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandRight;

impl Command for CommandRight {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "right".to_string(),
            "RIGHT".to_string(),
            "Right".to_string(),
            "r".to_string(),
            "R".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        get_mut_scene_manager().move_right();
    }
}