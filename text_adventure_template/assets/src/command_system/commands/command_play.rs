use crate::{command_system::{command_manager::get_mut_command_manager, command_schemes::CommandSchemes, commands::Command}, event_system::{event_manager::get_event_system, events::EventType}, save_system::save_system::get_save_system};

/// Immediately exits the application.
pub struct CommandPlay;

impl Command for CommandPlay {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "play".to_string(),
            "PLAY".to_string(),
            "Play".to_string(),
            "Start".to_string(),
            "start".to_string(),
            "START".to_string(),
            "p".to_string(),
            "P".to_string(),
            "s".to_string(),
            "S".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        //Set the commands to be gameplay
        get_mut_command_manager().active_commands_scheme = CommandSchemes::Gameplay;
        //Move to the scene listed as the last entered gameplay scene.
        get_event_system().invoke(EventType::OnMoveScenesRequest(get_save_system().get_current_scene()));
    }
}