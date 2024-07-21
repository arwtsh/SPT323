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
        println!("Welcome to DARK FOREST.");
        println!("\tPLAY | PROFILE | CREDITS | QUIT");

        get_mut_command_manager().active_commands_scheme = CommandSchemes::MainMenu;
    }
}