use crate::command_system::commands::Command;
use crate::command_system::command_manager::get_command_manager;

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
        //This will change based on the current command scheme.
        println!("{}", get_command_manager().active_commands_scheme.get_scheme_help_text());
    }
}