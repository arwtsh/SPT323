use std::collections::HashMap;

//
// ADD TO THE USES BELOW WHEN ADDING NEW COMMANDS
//
use crate::commands::command_exit;
use crate::commands::command_help;
use crate::commands::command_left;
use crate::commands::command_right;

//
// ADD TO THE ENUM BELOW WHEN ADDING NEW COMMANDS
//
///An ID for commands
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CommandId {
    None,
    Exit,
    Help,
    Left,
    Right
}

impl CommandId {
    pub fn to_string(&self) -> &str {
        match *self {
            CommandId::None => "None",
            CommandId::Exit => "Exit",
            CommandId::Help => "Help",
            CommandId::Left => "Left",
            CommandId::Right => "Right"
        }
    }
}

///Declares this as a command the player can call.
pub trait Command { //A trait makes a struct act more like a class with OOP.
    ///Get the string identities of this command.
    ///These will be used to match player text imput to a specific command.
    fn get_identifiers(&self) -> Vec<String>;

    ///Call the logic of this command.
    fn call_command(&self, params: &String);
}

/// The singleton of the CommandManager
static mut INSTANCE: Option<CommandManager> = Option::None;

/// Get the CommandManager singleton as immutable.
pub fn get_command_manager() -> &'static CommandManager {
    let command_manager: &CommandManager;
    unsafe {
        //Initialize CommandManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(CommandManager::init());
        }
        command_manager = INSTANCE.as_ref().unwrap();
    }
    command_manager
}

/// Get the CommandManager singleton as mutable.
pub fn get_mut_command_manager() -> &'static mut CommandManager {
    let command_manager: &mut CommandManager;
    unsafe {
        //Initialize CommandManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(CommandManager::init());
        }
        command_manager = INSTANCE.as_mut().unwrap();
    }
    command_manager
}

///Manages interpretation of player input.
pub struct CommandManager {
    //A hash map for every command. This is the first container that is populated.
    all_commands : HashMap<CommandId, Box<dyn Command>>,
    //Dictionary for storing all player input parses.
    command_identifiers : HashMap<String, CommandId>
}

impl CommandManager {
    pub fn init() -> CommandManager {
        //Declare a new instance of CommandManager
        CommandManager {
            all_commands: find_commands(), //Find every command and store it.
            command_identifiers: compile_command_parses(&find_commands()) //Initialize the command hash map.
        }
    }

    /// Convert a string to CommandId
    pub fn parse_command(&self, command: &String) -> CommandId{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        let result = self.command_identifiers.get(command); 
        if result.is_some() {
            *result.unwrap()
        } else {
            CommandId::None
        }
    }

    /// Get a Command by CommandId
    /// The command must have been connected in the find_commands function
    pub fn get_command(&self, command_id: CommandId) -> &Box<dyn Command> {
        if command_id != CommandId::None {
            let command = self.all_commands.get(&command_id);
            if command.is_some() {
                command.unwrap()
            } else {
                panic!("The command id {} did not have an associated command.", command_id.to_string())
            }
        } else {
            panic!("The command id \"None\" will never have an associated command.");
        }
    }
}

/// Compiles a HashMap to have all string-id pairs from command data.
fn compile_command_parses(all_commands: &HashMap<CommandId, Box<dyn Command>>) -> HashMap<String, CommandId>{
    let mut command_identifiers : HashMap<String, CommandId> = HashMap::new();
    
    //Loop through every command data.
    for command in all_commands.iter() {
        //Loop through the collection of strings in a command.
        for command_identifier in command.1.as_ref().get_identifiers() {
            if command_identifiers.contains_key(&command_identifier) {
                panic!("A command parse identifier \"{}\" in command {} already points to command {}", command_identifier, command.0.to_string(), command_identifiers.get(&command_identifier).unwrap().to_string());
            }
            else {
                //Populate the scene_parse hash map from the data in scene data.
                command_identifiers.insert(command_identifier.clone(), *command.0);
            }
        }
    }

    command_identifiers
}

/// Parses user input into command format and calls the command.
pub fn parse_user_input(input: &String) {
    let input_split = input.trim().split_once(' '); //split the string at the first space, so it should only be the first word
    if input_split.is_some() { //check if it split correctly
        let split_result = input_split.unwrap();
        interpret_command(&split_result.0.to_string(), &split_result.1.to_string());
    } else {
        //If it did not split, the command is all one word, such as "help" or "exit"
        interpret_command(input, &"".to_string());
    }
}


/// Interpret the split player input.
fn interpret_command(command: &String, params: &String) {
    let command_manager = get_command_manager();
    let command_id: CommandId= command_manager.parse_command(&command); //Convert the string to enum
    if command_id == CommandId::None { //Make sure the command the user typed exhists.
        println!("Command {} does not match any commands. Use \"help\" to list all the different commands.", command);
    }
    else {
        //Call the appropriate command.
        let command: &Box<dyn Command> = command_manager.get_command(command_id);
        command.as_ref().call_command(params);
    }
}

//
// MODIFY THE BELOW FUNCTIONS WHEN ADDING NEW COMMANDS
//

//Gets command data of every command and puts it in a hash map.
//The hash map makes it easily accessible and iterable.
fn find_commands() -> HashMap<CommandId, Box<dyn Command>> {
    let mut map: HashMap<CommandId, Box<dyn Command>>  = HashMap::new();
    map.insert(CommandId::Exit, Box::new(command_exit::CommandExit));
    map.insert(CommandId::Help, Box::new(command_help::CommandHelp));
    map.insert(CommandId::Left, Box::new(command_left::CommandLeft));
    map.insert(CommandId::Right, Box::new(command_right::CommandRight));
    map 
}