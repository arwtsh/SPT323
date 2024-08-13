use log::{error, info, warn};
use super::{file_io::{delete_file, does_file_exist, load_file, save_file}, profile_data::{ProfileData, DEFAULT_PROFILE, PROFILE_FILE_TYPE, PROFILE_SAVE_LOCATION}, save_system::SaveSystem};

/// A wrapper that stores a profile. 
/// This is used to have data attached to the profile that isn't saved to disk.
pub(super) struct ProfileWrapper {
    //The data belonging to this wrapper.
    //If this value is Option::None, then it is default.
    pub(super) content: Option<ProfileData>,
    //A flag that tells if the profile's data has changed. This is useful for autosaving.
    pub(super) has_changed: bool
}

impl SaveSystem {
    /// Get the profile data from the wrapper.
    pub(super) fn get_profile(&self) -> &ProfileData {
        //Return the stored content or default.
        self.get_profile_wrapper().content.as_ref().unwrap_or(&DEFAULT_PROFILE)
    }

    /// Get mutable profile data from the wrapper.
    /// Calling this will get rid of default.
    pub(super) fn get_mut_profile(&mut self) -> &mut ProfileData {
        //Since this is getting a mut variable, it can't return a constant.
        //Mutables borrows normally mean something will change, so it wouldn't be default anymore anyway.
        //Return the stored content or default.
        self.get_mut_profile_wrapper().content.get_or_insert(DEFAULT_PROFILE)
    }

    /// If the profile is default
    /// Profiles are default if they were not loaded from a file.
    pub fn is_profile_default(&self) -> bool {
        self.get_profile_wrapper().content.is_none()
    }

    /// Saves the ProfileData in memory to a file.
    pub fn save_profile(&self) {
        //Make sure the current profile actually exists. 
        //The only reason it shouldn't is because it hasn't been initialized yet.
        if self.get_current_profile().is_some() {
            info!("Saving profile {}", self.get_current_profile().unwrap());
            write_profile_data(&self.get_profile(), self.get_current_profile().unwrap());
        } else {
            warn!("Cannot save profile, profile name is marked as nonexistant.");
        }
    }

    /// Loads ProfileData from a file into memory.
    /// This doesn't allow a different profile to be loaded. 
    /// If the current profile variable in settings isn't set first, this will just re-load the current profile (potentially resetting progress).
    pub fn load_profile(&mut self) {
        //Make sure the current profile actually exists. 
        //The only reason it shouldn't is because it hasn't been initialized yet.
        if self.get_current_profile().is_some() {
            //Load the profile data
            info!("Loading profile {}", self.get_current_profile().unwrap());
            if let Some(result) = load_profile_data(self.get_current_profile().unwrap()) {
                self.get_mut_profile_wrapper().content = Some(result);
            }
        } else {
            warn!("Cannot save profile, profile name is marked as nonexistant.");
        }
    }

    /// Flag the profile data as default.
    /// Next time it is retrieved, it will return the default profile.
    pub fn reset_profile(&mut self) {
        println!("test");
        self.get_mut_profile_wrapper().content = None;
    }

    /// Checks if the current profile has a file saved on the local machine.
    /// This is a good way to check for new installs or file corruption.
    /// Returns false if the current profile name is marked as default (Option::None).
    pub fn does_current_profile_exist(&self) -> bool {
        match &self.get_current_profile() {
            Some(profile_name) => does_file_exist(&make_profile_path(&profile_name)),
            None => false //If the current profile name is marked as default, then it has not been saved.
        }
        
    }

    pub fn delete_profile(&mut self, profile_to_delete: &String) {
        //First, delete the file from the hard drive.
        delete_profile_data(profile_to_delete);

        info!("Deleted profile {}", profile_to_delete);

        //Then, handle what happens if that was the current profile.
        if self.get_current_profile().is_some_and(|name| name == profile_to_delete) {
            //Force the profile and current profile setting to default.
            self.get_mut_settings().current_profile = None;
            self.reset_profile();

            info!("Deleted profile was the current profile.");
        }
    }
}

/// Piece together the save file path from the profile name.
/// This is essentially a macro which expands to {PROFILE_SAVE_LOCATION + profile name}.PROFILE_FILE_TYPE
pub(super) fn make_profile_path(profile_name: &String) -> String {
    PROFILE_SAVE_LOCATION.to_string() + profile_name + "." + PROFILE_FILE_TYPE
}

/// Load the ProfileData from a path on the computer.
/// If the profile can't be loaded, it returns Option::None.
pub(super) fn load_profile_data(profile_name: &String) -> Option<ProfileData> {
    //Build profile path
    let path = make_profile_path(profile_name);

    //Check if the profile file exists.
    //If it doesn't then the profile doesn't exist yet, so we return an empty save data.
    if std::path::Path::new(&path).exists() {
        match load_file(&path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Result::Ok(profile) => Option::Some(profile),
                    Result::Err(e) => {error!("Profile {} at path {} could not be parsed. This could mean potential data corruption. \nError: {}", profile_name, path, e); Option::None}
                }
            },
            Err(e) => {
                error!("Profile {} could not be loaded at path {}. \nError: {}", profile_name, path, e); 
                Option::None
            }
        }
    } else {
        warn!("Profile {} could not be located at path {}.", profile_name, path);
        Option::None
    }
}

/// Write ProfileData to a file.
pub(super) fn write_profile_data(profile: &ProfileData, profile_name: &String) {
    //Build profile path
    let path = make_profile_path(profile_name);

    match serde_json::to_string(&profile) {
        Ok(json_data) => {
            //Write the serialized json to a file and handle errors.
            if let Err(e) = save_file(&path, &json_data) {
                error!("Unable to save profile {} to {}. Saving canceled. \nError: {}", profile_name, path, e)
            }
        },
        Err(e) => error!("Unable to serialize profile to JSON. Saving canceled. \nError: {}", e)
    }
}

/// Deletes a saved profile from the hard drive.
pub(super) fn delete_profile_data(profile_name: &String) {
    //Build profile path
    let path = make_profile_path(profile_name);

    if std::path::Path::new(&path).exists() {
        match delete_file(&path) {
            Ok(_) => info!("Deleted profile {}.", profile_name),
            Err(e) => error!("Profile {} could not be deleted from path {}. \nError: {}", profile_name, path, e)
        }
    } else {
        error!("The profile {} could not be found, deletion failed.", profile_name);
    }
}