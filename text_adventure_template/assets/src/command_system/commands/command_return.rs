use crate::{command_system::commands::Command, event_system::{event_manager::get_event_system, events::EventType}};

/// Returns to the main menu
pub struct CommandReturn;

impl Command for CommandReturn {
    fn get_identifiers(&self) -> Vec<String> {
        vec![ //populate the identifiers with string literals. These will be what is used to match player input this command.
            "exit".to_string(),
            "EXIT".to_string(),
            "Exit".to_string(),
            "Quit".to_string(),
            "quit".to_string(),
            "QUIT".to_string(),
            "RETURN".to_string(),
            "return".to_string(),
            "Return".to_string(),
            "q".to_string(),
            "Q".to_string(),
            "e".to_string(),
            "E".to_string()
        ]
    }
    fn call_command(&self, _params: &String) {
        //Return to main menu
        get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::MainMenu));
    }
}