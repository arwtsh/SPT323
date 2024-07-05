use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SETTINGS_PATH: &str = "saves/settings.save";
const PROFILE_SAVE_LOCATION: &str = "saves/"; //Must end in a /
const PROFILE_FILE_TYPE: &str = "save"; //Do not include the "."

/// The save data that represents the settings. 
/// Settings are independ of player profile.
#[derive(Deserialize, Serialize)]
struct SettingsData {
    //Volumes are a number from 0-100 that represents a %
    music_volume: i8,
    effects_volume: i8,
    environment_volume: i8,
    //If this application should be windowed maximized or fullscreen. 
    //If windowed maximized, the player is able to adjust the window size.
    is_fullscreen: bool,
    //The name of the profile the player is using. This is used to know which profile to initialy load on app launch.
    current_profile: String, 
    //A flag that tells if the settings have been changed. This is useful for autosaving.
    has_changed: bool
}

impl SettingsData {
    pub fn default() -> Self {
        SettingsData {
            music_volume: 50,
            effects_volume: 50,
            environment_volume: 50,
            is_fullscreen: false,
            current_profile: "default".to_string(),
            has_changed: false
        }
    }
}

/// The data structure that will be saved to the drive. 
/// Different SaveDatas will be on the hard drive for each player profile.
#[derive(Deserialize, Serialize)]
struct SaveData {
    pub world_data: WorldData,
    pub player_data: PlayerData,
    has_changed: bool //A flag that tells if the settings have been changed. This is useful for autosaving.
}

impl SaveData {
    /// Construct the default version of SaveData.
    /// Does not load any data.
    /// This is how a new save profile is created.
    pub fn default() -> Self {
        SaveData {
            world_data: WorldData {
                flags: HashMap::new()
            },
            player_data: PlayerData {
                current_scene: "start_scene".to_string()
            },
            has_changed: false
        }
    }
}

/// All of the save data that scenes will use, and other variables that represent the state of the world.
#[derive(Deserialize, Serialize)]
struct WorldData {
    //The primary way of storing the game state, in a large list of booleans. 
    //While it would be efficient to keep the booleans stored in a int and bit shift them, this is better to save to json.
    pub flags: HashMap<String, bool> 
}

/// All of the save data that represents the current player state
/// For example: the current scene the player is in, not counting a main menu or pause menu.
#[derive(Deserialize, Serialize)]
struct PlayerData {
    pub current_scene: String,
}

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
    save_data: SaveData,
    settings: SettingsData
}

impl SaveSystem {
    pub fn init() -> Self{
        let loaded_settings: SettingsData = load_settings(&SETTINGS_PATH.to_string());
        SaveSystem {
            save_data: load_save_data(&make_profile_path(&loaded_settings.current_profile)),
            settings: loaded_settings
        }
    }

    /// Changes the current profile name stored in the settings data.
    /// This does not immediately save or load data.
    /// To finish changing the profile, call load_profile afterward.
    pub fn change_profile(&mut self, new_profile: String) {
        self.settings.current_profile = new_profile;
    }

    /// Saves the SaveData in memory to the profile saved in SettingsData.
    /// To save to a new profile, first call change_profile.
    pub fn save_profile(&self) {
        write_save_data(&make_profile_path(&self.settings.current_profile), &self.save_data);
    }

    /// Loads SaveData from a file into memory.
    /// To load a different profile, first call change_profile.
    pub fn load_profile(&mut self) {
        self.save_data = load_save_data(&make_profile_path(&self.settings.current_profile));
    }

    /// Save the current settings to a file.
    /// There is no load_settings function. The settings are always stored in memory, it is only loaded on application startup.
    pub fn save_settings(&self) {
        write_settings_data(&SETTINGS_PATH.to_string(), &self.settings);
    }

    /// Get a flag in the save data.
    /// If the flag does not exhist, it returns false.
    pub fn get_flag(&self, flag: &String) -> bool {
        let result = self.save_data.world_data.flags.get(flag); //Get the flag from the world_data.
        if result.is_some() {
            *result.unwrap()
        } else {
            false
        }
    }

    /// Sets a world data flag.
    /// A flag would be like "secret_door_open"
    pub fn set_flag(&mut self, name: String, flag: bool) {
        self.save_data.world_data.flags.insert(name, flag); //Insert will create or update a key-value pair.
    }

    //To-do: Add getters for the variables in settings.
}

/// Load the SaveData from a path on the computer.
/// If the save profile doesn't exist, it creates a default save in memory.
/// If the save profile exists but was edited/corrupted in a way it can't load to the program will crash.
fn load_save_data(path: &String) -> SaveData {
    //Check if the save data exists.
    //If it doesn't then the profile doesn't exist yet, so we return an empty save data.
    if std::path::Path::new(path).exists() {
        let string_contents = load_file(path);
        let result: SaveData = serde_json::from_str(&string_contents).expect("Unable to parse save file.");
        result
    } else {
        SaveData::default() //Get the default save data.
    }
}

/// Load the SettingsData from a path on the computer.
/// If the save profile doesn't exist, it creates a default save in memory.
/// If the save profile exists but was edited/corrupted in a way it can't load to the program will crash.
fn load_settings(path: &String) -> SettingsData {
    //Check if the settings file exists.
    //If it doesn't then the player hasn't made any changes yet (or for some reason deleted the file), so we return the default.
    if std::path::Path::new(path).exists() {
        let string_contents = load_file(path);
        let result: SettingsData = serde_json::from_str(&string_contents).expect("Unable to parse settings file.");
        result
    } else {
        SettingsData::default() //Get the default settings.
    }
}

/// Load string contents of a file on the hard drive.
/// Any call to this function should be preceded by "if std::path::Path::new(&path).exists()" to handle what happens when the path isn't there.
/// This function will cause the program to crash if it doesn't find a file at that location or if it can't read it as a string.
fn load_file(path: &String) -> String {
    let data: String;
    data = std::fs::read_to_string(path).expect("Unable to read file");
    data
}

/// Piece together the save file path from the profile name.
/// This is essentially a macro which expands to {PROFILE_SAVE_LOCATION + profile name}.PROFILE_FILE_TYPE
fn make_profile_path(profile_name: &String) -> String {
    PROFILE_SAVE_LOCATION.to_string() + profile_name + "." + PROFILE_FILE_TYPE
}

/// Save string contents to a file of path.
/// This will create the file if it does not exist or completely rewrite if it does.
fn save_file(path: &String, contents: &String) {
    std::fs::write(path, contents).expect("Unable to write file");
}

/// Write the SaveData to a file.
/// If it is unable to for some reason, the program will crash.
fn write_save_data(path: &String, save_data: &SaveData) {
    let json_data = serde_json::to_string(&save_data).expect("Unable to serialize save data to JSON.");

    save_file(&path, &json_data);
}

/// Write the SettingsData to a file.
/// If it is unable to for some reason, the program will crash.
fn write_settings_data(path: &String, settigns: &SettingsData) {
    let json_data = serde_json::to_string(&settigns).expect("Unable to serialize save data to JSON.");

    save_file(path, &json_data);
}