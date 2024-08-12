use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType;
use crate::save_system::save_system::get_mut_save_system;
use crate::save_system::util::get_all_profiles;
use crate::command_system::commands::Command;

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandProfileLoad;

impl Command for CommandProfileLoad {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "load".to_string(),
            "Load".to_string(),
            "LOAD".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        let selection = _params.parse::<usize>();
        if selection.is_ok() {
            let profiles = get_all_profiles();
            let new_profile = profiles.get(selection.unwrap().max(1) - 1);
            if new_profile.is_some() {
                //Change the profile
                get_mut_save_system().set_current_profile(new_profile.unwrap());
                get_mut_save_system().load_profile();
                println!("Loaded profile: {}", new_profile.unwrap());

                //Transfer control back to the main menu
                get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::MainMenu))
            } else {
                println!("That selection does not exist");
            }
        } else {
            println!("Please enter the number of your selection.");
        }
    }
}