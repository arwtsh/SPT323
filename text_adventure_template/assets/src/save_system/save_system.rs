use log::warn;

use super::{profile_data::PROFILE_SAVE_LOCATION, profile_wrapper::{load_profile_data, ProfileWrapper}, settings_data::DEFAULT_SETTINGS, settings_wrapper::{load_settings_data, SettingsWrapper}};

/// The singleton of the SaveSystem
static mut INSTANCE: Option<SaveSystem> = Option::None;

/// Get the SaveSystem singleton as immutable.
pub fn get_save_system() -> &'static SaveSystem {
    let save_system: &SaveSystem;
    unsafe {
        //Initialize SaveSystem if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(SaveSystem::init());
        }
        save_system = INSTANCE.as_ref().unwrap();
    }
    save_system
}

/// Get the SaveSystem singleton as mutable.
pub fn get_mut_save_system() -> &'static mut SaveSystem {
    let save_system: &mut SaveSystem;
    unsafe {
        //Initialize SaveSystem if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(SaveSystem::init());
        }
        save_system = INSTANCE.as_mut().unwrap();
    }
    save_system
}

pub struct SaveSystem {
    profile: ProfileWrapper,
    settings: SettingsWrapper
}

impl SaveSystem {
    pub fn init() -> Self{
        //Create the save directory just in case it doesn't exist
        let _ = std::fs::create_dir_all(PROFILE_SAVE_LOCATION); 

        //Load the settings
        let settings_data = load_settings_data();
        let settings = SettingsWrapper { //If the settings is default, then it should save to the disk, since that means it didn't load successfully.
            has_changed: settings_data.is_none(),
            content: settings_data
        };

        //Check if the settings has a current profile and load it.
        let profile_data = match &settings.content.as_ref().unwrap_or(&DEFAULT_SETTINGS).current_profile {
            Some(profile_name) => {
                load_profile_data(&profile_name)
            },
            None => {
                warn!("No initial profile specified in settings. Loading default profile.");
                Option::None
            }
        };
        let profile = ProfileWrapper { //A profile should never be saved at this point. 
            content: profile_data,
            has_changed: false
        };

        //Construct the save system struct.
        SaveSystem {
            profile: profile,
            settings: settings
        }
    }

    pub(super) fn get_settings_wrapper(&self) -> &SettingsWrapper {
        &self.settings
    }

    pub(super) fn get_mut_settings_wrapper(&mut self) -> &mut SettingsWrapper {
        &mut self.settings
    }

    pub(super) fn get_profile_wrapper(&self) -> &ProfileWrapper {
        &self.profile
    }

    pub(super) fn get_mut_profile_wrapper(&mut self) -> &mut ProfileWrapper {
        &mut self.profile
    }
}

/*
pub fn create_new_profile() {
    let profile_name = get_profile_name_from_input();

    //Change the profile and immediately save it.
    get_mut_save_system().change_profile(&profile_name);
    get_save_system().save_profile();
}

/// Get the name for a profile from the player.
/// If the player types in an illegal name it will prompt them again.
fn get_profile_name_from_input() -> String {
    let mut is_valid_name: bool = true;
    let mut profile_name: String = String::new();

    //Loop so that in case of an error, the player can try again.
    while is_valid_name {

        println!("New profile name:");

        //Get the profile name from user input.
        profile_name =  user_input::get_user_input();

        //Make sure the profile is in the correct format.
        let validity = is_valid_profile_name(&profile_name);
        is_valid_name = !validity.0;

        //Print the reason for invalid name
        if !is_valid_name {
            println!("{}", validity.1);
        }
    }

    profile_name
}
*/