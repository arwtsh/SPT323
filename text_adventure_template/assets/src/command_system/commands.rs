pub mod command_credits;
pub mod command_exit;
pub mod command_help;
pub mod command_left;
pub mod command_play;
pub mod command_profile;
pub mod command_right;

use std::slice::Iter;

///An ID for commands
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CommandId {
    None,
    Exit,
    Help,
    Left,
    Right,
    Play,
    Credits,
    Profile
}

impl CommandId {
    /// Convert the command ID to a string.
    /// This is useful for debugging.
    pub fn to_string(&self) -> &str {
        match *self {
            CommandId::None => "None",
            CommandId::Exit => "Exit",
            CommandId::Help => "Help",
            CommandId::Left => "Left",
            CommandId::Right => "Right",
            CommandId::Credits => "Credits",
            CommandId::Play => "Play",
            CommandId::Profile => "Profile"
        }
    }

    /// Get the command associated with the ID.
    pub fn get_command(&self) -> Box<dyn Command> {
        match *self {
            CommandId::None => panic!("Command \"None\" is not a command, it is a placeholder that represents no command."),
            CommandId::Exit => Box::new(command_exit::CommandExit),
            CommandId::Help => Box::new(command_help::CommandHelp),
            CommandId::Left => Box::new(command_left::CommandLeft),
            CommandId::Right => Box::new(command_right::CommandRight),
            CommandId::Credits => Box::new(command_credits::CommandCredits),
            CommandId::Play => Box::new(command_play::CommandPlay),
            CommandId::Profile => Box::new(command_profile::CommandProfile)
        }
    }

    /// Get an iterator of all the commands.
    /// This is useful for initializing all the commands at game start.
    /// This does not include the None command.
    pub fn iter() -> Iter<'static, CommandId> {
        static COMMANDS: [CommandId; 7] = [
            CommandId::Exit,
            CommandId::Help,
            CommandId::Left,
            CommandId::Right,
            CommandId::Credits,
            CommandId::Play,
            CommandId::Profile
        ];
        COMMANDS.iter()
    }
}

/// Declares this as a command the player can call.
pub trait Command { //A trait makes a struct act more like a class with OOP.
    ///Get the string identities of this command.
    ///These will be used to match player text imput to a specific command.
    fn get_identifiers(&self) -> Vec<String>;

    ///Call the logic of this command.
    fn call_command(&self, params: &String);
}