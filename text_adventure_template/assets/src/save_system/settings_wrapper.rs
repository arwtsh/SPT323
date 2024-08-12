use log::{error, info};

use super::{file_io::{load_file, save_file}, save_system::SaveSystem, settings_data::{SettingsData, DEFAULT_SETTINGS, SETTINGS_PATH}};

/// A wrapper that stores settings. 
/// This is used to have data attached to settings that isn't saved to disk.
pub(super) struct SettingsWrapper {
    //The settings belonging to this wrapper.
    //If this value is Option::None, then it is default.
    pub(super) content: Option<SettingsData>,
    //A flag that tells if the settings have been changed. This is useful for autosaving.
    pub(super) has_changed: bool
}

impl SaveSystem {
    /// Get the settings data from the wrapper.
    pub fn get_settings(&self) -> &SettingsData {
        //Return the stored content or default.
        self.get_settings_wrapper().content.as_ref().unwrap_or(&DEFAULT_SETTINGS)
    }

    /// Get mutable settings data from the wrapper.
    /// Calling this will get rid of default.
    pub fn get_mut_settings(&mut self) -> &mut SettingsData {
        //Since this is getting a mut variable, it can't return a constant.
        //Mutables borrows normally mean something will change, so it wouldn't be default anymore anyway.
        //Return the stored content or default.
        self.get_mut_settings_wrapper().content.get_or_insert(DEFAULT_SETTINGS)
    }

    /// If the settings are default
    /// Settigns are default if they were not loaded from a file.
    pub fn is_settings_default(&self) -> bool {
        self.get_settings_wrapper().content.is_none()
    }

    /// Flag the profile data as default.
    /// Next time it is retrieved, it will return the default profile.
    pub fn reset_settings(&mut self) {
        self.get_mut_settings_wrapper().content = None;
    }

    /// Saves the SettingsData in memory to a file.
    pub fn save_settings(&self) {
        info!("Saving settings");
        write_settings_data(self.get_settings());
    }

    /// Loads SettingsData from a file into memory.
    pub fn load_settings(&mut self) {
        info!("Loading settings");
        if let Some(result) = load_settings_data() {
            self.get_mut_settings_wrapper().content = Some(result);
        }
    }
}

/// Load the SettingsData from a path on the computer.
/// If there is no save file, it returns Option::None.
pub(super) fn load_settings_data() -> Option<SettingsData> {
    //Check if the settings file exists.
    //If it doesn't then the player hasn't made any changes from default yet (or for some reason deleted the file), so we return the default.
    if std::path::Path::new(SETTINGS_PATH).exists() {
        match load_file(&SETTINGS_PATH.to_string()) {
            Result::Ok(content) => {
                match serde_json::from_str(&content) {
                    Result::Ok(settings) => Option::Some(settings),
                    Result::Err(e) => {error!("Settings could not be parsed. This could mean potential data corruption. \nError: {}", e); Option::None}
                }
            },
            Result::Err(e) => {error!("Settings could not be loaded at path {}. \nError: {}", SETTINGS_PATH, e); Option::None}
        }
    } else {
        error!("Settings file could not be located at path {}.", SETTINGS_PATH);
        Option::None
    }
}


/// Write SettingsData to a file.
pub(super) fn write_settings_data(settings: &SettingsData) {
    match serde_json::to_string(&settings) {
        Ok(json_data) => {
            //Write the serialized json to a file and handle errors.
            if let Err(e) = save_file(&SETTINGS_PATH.to_string(), &json_data) {
                error!("Unable to save settings to {}. Saving canceled. \nError: {}", SETTINGS_PATH, e)
            }
        },
        Err(e) => error!("Unable to serialize save data to JSON. Saving canceled. \nError: {}", e)
    }
}