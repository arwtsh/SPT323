use serde::{Deserialize, Serialize};

use super::save_system::SaveSystem;

pub(super) const SETTINGS_PATH: &str = "saves/settings.set";
pub(super) const DEFAULT_SETTINGS: SettingsData = SettingsData {
    music_volume: 50,
    effects_volume: 50,
    environment_volume: 50,
    is_fullscreen: false,
    current_profile: Option::None
};

/// The save data that represents the settings. 
/// Settings are independ of player profile.
#[derive(Deserialize, Serialize)]
pub struct SettingsData {
    //Volumes are a number from 0-100 that represents a %
    music_volume: i8,
    effects_volume: i8,
    environment_volume: i8,
    //If this application should be windowed maximized or fullscreen. 
    //If windowed maximized, the player is able to adjust the window size.
    is_fullscreen: bool,
    //The name of the profile the player is using. This is used to know which profile to initialy load on app launch.
    //Option::None represents no current profile selected. This means the profile will be the default and the player needs to name it.
    pub(super) current_profile: Option<String>
}

impl SaveSystem {
    /// Get the name of the current profile selected.
    /// This should be used for detecting the profile loaded on setup.
    /// The value 'Option::None' means the default profile should be loaded then the player be forced into the profile select menu.
    pub fn get_current_profile(&self) -> Option<&String> {
        self.get_settings().current_profile.as_ref()
    }

    /// Sets the value of the current profile.
    /// This does not have any logic for changing the profile, it's just a setter.
    pub fn set_current_profile(&mut self, new_profile_name: &String) {
        self.get_mut_settings().current_profile = Some(new_profile_name.clone());
        //self.get_mut_settings_wrapper().has_changed = true;
        self.save_settings();
    }
}