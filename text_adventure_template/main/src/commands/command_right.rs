use crate::game_manager::Managers;
use crate::command_manager::CommandData;

///Get the CommandData for this command.
pub fn get_command_data() -> CommandData {
    CommandData{ //Create new command data.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "right".to_string(),
            "RIGHT".to_string(),
            "Right".to_string(),
            "r".to_string(),
            "R".to_string()
        ],
    }
}

pub fn call_command(params: String, managers: &mut Managers) {
    managers.get_scene_manager().move_right();
}