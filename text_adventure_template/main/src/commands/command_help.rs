use crate::game_manager::Managers;
use crate::command_manager::CommandData;

///Get the CommandData for this command.
pub fn get_command_data() -> CommandData {
    CommandData{ //Create new command data.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "help".to_string(),
            "HELP".to_string(),
            "Help".to_string(),
            "h".to_string(),
            "H".to_string()
        ],
    }
}

pub fn call_command(params: String, managers: &Managers) {
    println!("HELP!");
}