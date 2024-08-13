use log::warn;
use serde::{Deserialize, Serialize};
use crate::scene_system::scene_id::{SceneId, STARTING_SCENE};

use super::save_system::SaveSystem;

pub(super) const DEFAULT_PROFILE_PLAYER: PlayerData = PlayerData { //The player portion of the default profile data.
    current_scene: STARTING_SCENE
};

/// All of the save data that represents the current player state
/// For example: the current scene the player is in, not counting a main menu or pause menu.
#[derive(Deserialize, Serialize)]
pub(super)  struct PlayerData {
    //The current "gameplay" scene the player is in. 
    //This does not count scenes like the main menu. 
    //This variable is not referenced during runtime, only by loading saves.
    pub current_scene: SceneId
}

impl SaveSystem {
    /// Gets the current scene the player is in.
    /// This only counts gameplay scenes, not the Main Menu for example.
    pub fn get_current_scene(&self) -> SceneId {
        self.get_profile().player_data.current_scene
    }

    /// Sets the current scene the player is in.
    /// If the scene is not marked as saveable, nothing happens.
    pub fn set_current_scene(&mut self, new_scene: SceneId) {
        //Only update if the scene is saveable.
        if new_scene.is_saveable() {
            self.get_mut_profile().player_data.current_scene = new_scene;
            //self.get_mut_profile_wrapper().has_changed = true;
            self.save_profile();
        }
    }
}