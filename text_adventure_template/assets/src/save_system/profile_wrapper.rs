use log::{error, info, warn};
use super::{file_io::{delete_file, does_file_exist, load_file, save_file}, profile_data::{ProfileData, DEFAULT_PROFILE, PROFILE_FILE_TYPE, PROFILE_MAX_LENGTH, PROFILE_SAVE_LOCATION}, save_system::SaveSystem};

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
    pub fn get_profile(&self) -> &ProfileData {
        //Return the stored content or default.
        self.get_profile_wrapper().content.as_ref().unwrap_or(&DEFAULT_PROFILE)
    }

    /// Get mutable profile data from the wrapper.
    /// Calling this will get rid of default.
    pub fn get_mut_profile(&mut self) -> &mut ProfileData {
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

    /// Check if the provided string is a valid option for a save file name.
    /// It is important this is sanitized since it could cause the program to try and save an illegal file, causing the program to crash.
    pub fn is_valid_profile_name(profile_name: &String) -> (bool, String) {
        let chars = profile_name.chars();
        let chars_count = profile_name.chars().count();

        //Profile name can't be empty.
        if profile_name.is_empty() {
            return (false, "Profile names must not be empty.".to_string());
        }

        //Make sure the player can't put too long of a profile name into the file system.
        if chars_count > PROFILE_MAX_LENGTH.into() {
            return (false, "The profile name is too long.".to_string());
        }

        //Check each character to see if it's an allowed file system character.
        for char in chars {
            if !char.is_alphanumeric() {
                return (false, "The profile name can only contain letters and numbers.".to_string());
            }
        }

        (true, String::new())
    }

    /// Get a list of all available profiles.
    /// This is done by reading the filenames in the save directory.
    pub fn get_all_profiles() -> Vec<String> {
        let mut profiles: Vec<String> = Vec::new();
        //Get all files where the filename matches the expected save file naming system.
        let files = glob::glob(&(PROFILE_SAVE_LOCATION.to_string() +  "/*." + PROFILE_FILE_TYPE)).unwrap().filter_map(Result::ok); 
        for file in files {
            //if file.file_name()
            profiles.push(file.file_name().unwrap().to_str().unwrap().split_once('.').unwrap().0.to_string());
        }
        profiles
    }
}

/// Piece together the save file path from the profile name.
/// This is essentially a macro which expands to {PROFILE_SAVE_LOCATION + profile name}.PROFILE_FILE_TYPE
fn make_profile_path(profile_name: &String) -> String {
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