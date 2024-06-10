use crate::game_manager::Managers;
use crate::command_manager::CommandData;

///Get the CommandData for this command.
pub fn get_command_data() -> CommandData {
    CommandData{ //Create new command data.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
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
        ],
    }
}

pub fn call_command(params: String, managers: &Managers) {
    println!("EXIT!");
}