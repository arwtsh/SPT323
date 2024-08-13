use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}};

/// Quits the profile menu and returns control to MainMenu.
pub struct CommandProfileCancel;

impl Command for CommandProfileCancel {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "cancel".to_string(),
            "CANCEL".to_string(),
            "Cancel".to_string(),
            "c".to_string(),
            "C".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::MainMenu))
    }
}