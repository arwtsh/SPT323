use crate::command_manager::Command;

/// Displays information about how to play this game.
pub struct CommandHelp;

impl Command for CommandHelp {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "help".to_string(),
            "HELP".to_string(),
            "Help".to_string(),
            "h".to_string(),
            "H".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        //Display to the terminal how to play the game and the commands to use.
        println!("You will be given a text description of a scene.");
        println!("You will chose to go either RIGHT or LEFT.");
        println!("HELP repeats these tips.");
        println!("Help closes the game.");
    }
}