use scene_util::scene_template::Scene;
use crate::commands::command_schemes::CommandSchemes;

use crate::command_manager::get_mut_command_manager;


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