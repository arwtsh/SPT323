use std::collections::HashSet;

use crate::command_system::commands::CommandId;

#[derive( Clone, Copy)]
pub enum CommandSchemes {
    MainMenu,
    Gameplay
}

impl CommandSchemes {
    pub fn is_command_member(&self, command: &CommandId) -> bool {
        *command != CommandId::None && self.get_scheme_commands().get(&command).is_some()
    }

    fn get_scheme_commands(&self) -> HashSet<CommandId> {
        match self {
            CommandSchemes::MainMenu => {
                HashSet::from_iter(vec![
                    CommandId::Exit,
                    CommandId::Play,
                    CommandId::Credits,
                    CommandId::Profile,
                    CommandId::Help
                ])
            },
            CommandSchemes::Gameplay => {
                HashSet::from_iter(vec![
                    CommandId::Left,
                    CommandId::Right,
                    CommandId::Help
                ])
            }
        }
    }

    pub fn get_scheme_help_text(&self) -> &str {
        match self {
            CommandSchemes::MainMenu => 
            "HELP will tell you the commands you can use.
            \nPLAY will start the game.
            \nPROFILE allows you switch between saves of the game.
            \nCREDITS displays the creator of the game.
            \nQUIT will exit the game to desktop.",
            CommandSchemes::Gameplay => 
            "You will be given a text description of a scene.
            \nYou will chose to go either RIGHT or LEFT.
            \nHELP repeats these tips.
            \nQUIT closes the game."
        }
    }
}

