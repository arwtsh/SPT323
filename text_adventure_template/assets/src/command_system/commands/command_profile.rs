use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}};

/// A way for the player to move locations.
/// Moves the player to the scene marked "right"
pub struct CommandProfile;

impl Command for CommandProfile {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "profile".to_string(),
            "Profile".to_string(),
            "PROFILE".to_string(),
            "Pr".to_string(),
            "PR".to_string(),
            "pr".to_string()
        ]
    }

    fn call_command(&self, _params: &String) {
        get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::ProfileSelect))
    }
}