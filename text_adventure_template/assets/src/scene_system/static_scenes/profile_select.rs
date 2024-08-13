use crate::event_system::event_manager::EventSystem;
use crate::save_system::save_system::get_save_system;
use crate::save_system::util::get_all_profiles;
use crate::scene_system::scene_template::Scene;
use crate::command_system::command_manager::get_mut_command_manager;
use crate::command_system::command_schemes::CommandSchemes;

pub struct ProfileSelect;

impl ProfileSelect {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for ProfileSelect {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        //Print out the profiles on the drive
        println!("All profiles: ");
        let all_profiles = get_all_profiles();
        if all_profiles.is_empty() {
            println!("No profiles found.");
        } else {
            for i in 0..all_profiles.len() {
                println!("{}. {}", i + 1, all_profiles[i]);
            }
        }

        if get_save_system().does_current_profile_exist() {
            println!("Current profile: {}", get_save_system().get_current_profile().unwrap());
        } else {
            println!("No profile selected.");
        }
        
        println!("LOAD <profile> | NEW <name> | DELETE <profile> | CANCEL");

        get_mut_command_manager().active_commands_scheme = CommandSchemes::ProfileSelect;
    }
}