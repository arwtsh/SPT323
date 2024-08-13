use log::info;

use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}, save_system::{save_system::get_mut_save_system, util::is_valid_profile_name}};

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandProfileNew;

impl Command for CommandProfileNew {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "new".to_string(),
            "New".to_string(),
            "NEW".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        if let Some(error) = is_valid_profile_name(_params) {
            println!("{}", error); //Display why the profile name isn't valid to the player.
        } else {
            //Create new profile.
            get_mut_save_system().reset_profile();
            get_mut_save_system().set_current_profile(_params);
            get_mut_save_system().save_profile();
            println!("Created new profile: {}", _params);
            info!("Created new profile: {}", _params);

            //Close out of the profile window and back to the main menu.
            get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::MainMenu));
        }
    }
}