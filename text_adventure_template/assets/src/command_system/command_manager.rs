use std::collections::HashMap;
use command_schemes::CommandSchemes;
use crate::command_system::command_schemes;
use log::{error, info};
use crate::command_system::commands::{CommandId, Command};

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
    //Each command scheme has it's own parse map.
    command_identifiers : HashMap<CommandSchemes, HashMap<String, CommandId>>,
    //A scheme is all the commands that the player is allowed to use
    //This represents different parts of the game.
    pub active_commands_scheme: CommandSchemes
}

impl CommandManager {
    pub fn init() -> CommandManager {
        //Declare a new instance of CommandManager
        CommandManager {
            all_commands: find_commands(), //Find every command and store it.
            command_identifiers: compile_command_parses(&find_commands()), //Initialize the command hash map.
            active_commands_scheme: CommandSchemes::MainMenu //Start in the main menu
        }
    }

    /// Convert a string to CommandId
    pub fn parse_command(&self, command: &String) -> &CommandId{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        match self.command_identifiers.get(&self.active_commands_scheme) {
            Some(command_parses) => {
                match command_parses.get(command) {
                    Some(result) => result,
                    None => &CommandId::None
                }
            },
            None => &CommandId::None
        }
    }

    /// Get a Command by CommandId
    /// The command must have been connected in the find_commands function
    pub fn get_command(&self, command_id: &CommandId) -> &Box<dyn Command> {
        if command_id != &CommandId::None {
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
fn compile_command_parses(all_commands: &HashMap<CommandId, Box<dyn Command>>) -> HashMap<CommandSchemes, HashMap<String, CommandId>>{
    let mut result: HashMap<CommandSchemes, HashMap<String, CommandId>> = HashMap::new();
    for scheme in CommandSchemes::iter() {
        //Insert into the hashmap and catch any errors.
        if result.insert(scheme.clone(), scheme.get_scheme_parses(all_commands)).is_some() {
            error!("Command scheme {} found a duplicate of itself while compiling command parses.", scheme.to_string())
        }
    }
    
    result
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
    let command_id: &CommandId = command_manager.parse_command(&command); //Convert the string to enum
    if command_id == &CommandId::None { //Make sure the command the user typed exists and is allowed.
        println!("Command {} does not match any commands. Use \"help\" to list all the different commands.", command);
    }
    else {
        //Call the appropriate command.
        let command: &Box<dyn Command> = command_manager.get_command(command_id);
        info!("Calling command {}", command_id.to_string());
        command.as_ref().call_command(params);
    }
}

/// Gets command data of every command and puts it in a hash map.
/// The hash map makes it easily accessible and iterable.
fn find_commands() -> HashMap<CommandId, Box<dyn Command>> {
    let mut map: HashMap<CommandId, Box<dyn Command>>  = HashMap::new();
    for command_id in CommandId::iter() {
        map.insert(*command_id, command_id.get_command());
    }
    map
}