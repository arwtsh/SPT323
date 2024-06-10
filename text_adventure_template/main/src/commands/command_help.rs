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

///Run the logic of this command
pub fn call_command(params: String, managers: &Managers) {
    //Display to the terminal how to play the game and the commands to use.
    println!("You will be given a text description of a scene.");
    println!("You will chose to go either RIGHT or LEFT.");
    println!("HELP repeats these tips.");
    println!("EXIT closes the game.");
}