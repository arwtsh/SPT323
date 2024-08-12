use super::{file_io::does_file_exist, profile_data::{PROFILE_FILE_TYPE, PROFILE_MAX_LENGTH, PROFILE_SAVE_LOCATION}, profile_wrapper::make_profile_path};

///
/// A public module to put utility functions that should be exposed.
/// 

/// Check if the provided string is a valid option for a save file name.
/// It is important this is sanitized since it could cause the program to try and save an illegal file, causing the program to crash.
/// The value 'Option::None' is true. Some contains the error message.
pub fn is_valid_profile_name(profile_name: &String) -> Option<String> {
    let chars = profile_name.chars();
    let chars_count = profile_name.chars().count();

    //Profile name can't be empty.
    if profile_name.is_empty() {
        Some("Profile names must not be empty.".to_string());
    }

    //Make sure the player can't put too long of a profile name into the file system.
    if chars_count > PROFILE_MAX_LENGTH.into() {
        Some("The profile name is too long.".to_string());
    }

    //Check each character to see if it's an allowed file system character.
    for char in chars {
        if !char.is_alphanumeric() {
            Some("The profile name can only contain letters and numbers.".to_string());
        }
    }

    if does_file_exist(&make_profile_path(profile_name)) {
        Some("The profile name already belongs to an existing profile.".to_string());
    }

    None
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