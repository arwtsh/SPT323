use serde::{Deserialize, Serialize};
use crate::scene_system::scene_id::{SceneId, STARTING_SCENE};

pub(super) const DEFAULT_PROFILE_PLAYER: PlayerData = PlayerData { //The player portion of the default profile data.
    current_scene: STARTING_SCENE
};

/// All of the save data that represents the current player state
/// For example: the current scene the player is in, not counting a main menu or pause menu.
#[derive(Deserialize, Serialize)]
pub(super)  struct PlayerData {
    //The current scene the player is in.
    pub current_scene: SceneId
}