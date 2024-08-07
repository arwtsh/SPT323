use crate::save_system::get_save_system;
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
        check_profile(); //Make sure the player has a healthy and existing save profile.

        println!("Welcome to DARK FOREST.");
        println!("\tPLAY | PROFILE | CREDITS | QUIT");

        get_mut_command_manager().active_commands_scheme = CommandSchemes::MainMenu;
    }
}

fn check_profile() {
    if get_save_system().does_current_profile_exist() {
        
    }
}