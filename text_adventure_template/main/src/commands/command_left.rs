use crate::game_manager::Managers;
use crate::command_manager::CommandData;

///Get the CommandData for this command.
pub fn get_command_data() -> CommandData {
    CommandData{ //Create new command data.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "left".to_string(),
            "LEFT".to_string(),
            "Left".to_string(),
            "l".to_string(),
            "L".to_string()
        ],
    }
}

///Run the logic for this command
pub fn call_command(params: String, managers: &mut Managers) {
    managers.get_scene_manager().move_left();
}