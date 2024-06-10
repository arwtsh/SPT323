use std::collections::HashMap;
use crate::commands::command_help;
use crate::commands::command_exit;


mod commands {
    pub mod command_exit;
    pub mod command_help;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum CommandId {
    None,
    Exit,
    Help
}

impl CommandId {
    pub fn to_string(&self) -> &str {
        match *self {
            CommandId::None => "None",
            CommandId::Exit => "Exit",
            CommandId::Help => "Help"
        }
    }
}

pub struct CommandData {
    identifiers: Vec<String>
}

///Manages interpretation of player input.
pub struct CommandManager {
    all_command_data : HashMap<CommandId, CommandData>,
    //Dictionary for storing all player input parses.
    all_commands : HashMap<String, CommandId>
}

impl CommandManager {
    pub fn init() -> CommandManager {
        //Declare a new instance of CommandManager
        let mut manager = CommandManager {
            all_command_data: find_command_data(), //Find every CommandData and store it.
            all_commands: HashMap::new() //Initialize the command hash map.
        };
        manager.compile_command_parses(); //Populate the command hash map.
        manager //Return the new instance.
    }

    ///Compiles the HashMap to have all string-id pairs from command data.
    fn compile_command_parses(&mut self) {
        //Loop through every command data.
        for command_data in self.all_command_data.iter() {
            //Loop through the collection of strings in an command data.
            for command_identifier in &command_data.1.identifiers {
                if self.all_commands.contains_key(command_identifier) {
                    panic!("A command parse identifier \"{}\" in command {} already points to command {}", command_identifier, command_data.0.to_string(), self.all_commands.get(command_identifier).unwrap().to_string());
                }
                else {
                    //Populate the scene_parse hash map from the data in scene data.
                    self.all_commands.insert(command_identifier.clone(), *command_data.0);
                }
            }
        }
    }

    ///Parses user input into command format and calls the command.
    pub fn parse_user_input(&self, input: String) {
        let input_split = input.trim().split_once(' ');
        if input_split.is_some() { //check if it split correctly
            let split_result = input_split.unwrap();
            self.interpret_command(split_result.0.to_string(), split_result.1.to_string());
        } else {
            //If it did not split, the command is all one word, such as "help" or "exit"
            self.interpret_command(input, "".to_string());
        }
    }

    //convert a string to CommandId
    fn parse_command(&self, command: &String) -> CommandId{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        let result = self.all_commands.get(command); 
        if result.is_some() {
            *result.unwrap()
        } else {
            CommandId::None
        }
    }

    ///Interpret the split player input.
    fn interpret_command(&self, command: String, params: String) {
        let command_id: CommandId = self.parse_command(&command); //Convert the string to enum
        if command_id == CommandId::None { //Make sure the command the user typed exhists.
            println!("Command {} does not match any commands. Use \"help\" to list all the different commands.", command);
        }
        else {
            //Call the appropriate command.
            call_command(command_id, params);
        }
    }
}

//
// MODIFY THE BELOW FUNCTIONS WHEN ADDING NEW COMMANDS
//

//Gets command data of every command and puts it in a hash map.
//The hash map makes it easily accessible.
fn find_command_data() -> HashMap<CommandId, CommandData> {
    let mut map = HashMap::new();
    map.insert(CommandId::Help, command_help::get_command_data());
    map.insert(CommandId::Exit, command_exit::get_command_data());
    map
}

//Until I know how to do OOP, I have to call functions individually.
fn call_command(command: CommandId, input: String) {
    match command {
        CommandId::None => println!("Command does not exhist."),
        CommandId::Exit => command_exit::call_command(input),
        CommandId::Help => command_help::call_command(input)
    }
}