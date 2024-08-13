use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::save_system::SaveSystem;

pub(super) const DEFAULT_PROFILE_WORLD: WorldData = WorldData { //The world portion of the default profile data.
    flags: Option::None
};

/// All of the save data that scenes will use, and other variables that represent the state of the world.
#[derive(Deserialize, Serialize)]
pub(super) struct WorldData {
    //The primary way of storing the game state, in a large list of booleans. 
    //While it would be efficient to keep the booleans stored in a int and bit shift them, this is better to save to json.
    pub flags: Option<HashMap<String, bool>> 
}

impl SaveSystem {
    /// Get a flag in the world data.
    /// A flag is a boolean that when true represents progression in the world.
    /// For example: isDoorOpen
    /// If the flag does not exist, it returns false.
    pub fn get_flag(&self, flag: &String) -> bool {
        //Get the flag from the world_data.
        match &self.get_profile().world_data.flags {
            Some(map) => {
                match map.get(flag) {
                    Some(result) => result.clone(),
                    None => false
                }
            },
            None => false
        }  
    }

    /// Sets a world data flag.
    /// A flag is a boolean that when true represents progression in the world.
    /// For example: isDoorOpen
    pub fn set_flag(&mut self, name: String, flag: bool) {
        //Initialize the flags if it is completely empty.
        if self.get_profile().world_data.flags.is_none() {
            let _ = &self.get_mut_profile().world_data.flags.insert(HashMap::new());
        }

        //Get the flags, initialize it if it's default to Option::None, insert the new flag.
        //Then check if the flag was actually updated, and if so, flag the wrapper that the profile changed.
        if match self.get_mut_profile().world_data.flags.get_or_insert(HashMap::new()).insert(name, flag) {
            Some(old_value) => flag != old_value,
            None => true
        } {
            //self.get_mut_profile_wrapper().has_changed = true;
            self.save_profile();
        }
    }
}