use crate::event_system::event_manager::get_event_system;
use crate::event_system::events::EventType;
use crate::save_system::save_system::get_save_system;
use crate::scene_system::scene_template::Scene;
use crate::command_system::command_manager::get_mut_command_manager;
use crate::command_system::command_schemes::CommandSchemes;

pub struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for MainMenu {
    fn enter_scene(&self) {
        get_mut_command_manager().active_commands_scheme = CommandSchemes::MainMenu;

        println!("DARK FOREST.");

        //Check if current profile is invalid.
        //This may be because the player is a fresh instal, or because they deleted the current profile.
        if !get_save_system().does_current_profile_exist(){ 
            //Force the player into profile select.
            println!("Welcome, new player!");
            get_event_system().invoke(EventType::OnMoveScenesRequest(crate::scene_system::scene_id::SceneId::ProfileSelect));
            return;
        }
        else {
            println!("Welcome, {}", get_save_system().get_current_profile().unwrap());
        }

        println!("\tPLAY | PROFILE | CREDITS | QUIT");
    }
}