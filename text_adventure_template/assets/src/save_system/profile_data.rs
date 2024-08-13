use serde::{Deserialize, Serialize};

use super::{player_data::{PlayerData, DEFAULT_PROFILE_PLAYER}, world_data::{WorldData, DEFAULT_PROFILE_WORLD}};

pub(super) const PROFILE_SAVE_LOCATION: &str = "saves/"; //Must end in a /
pub(super) const PROFILE_FILE_TYPE: &str = "sav"; //Do not include the "."
pub(super) const PROFILE_MAX_LENGTH: u8 = 16; //The maximum length of the profile's file name. This does not include the file type.
pub(super) const DEFAULT_PROFILE: ProfileData = ProfileData { //The default profile contents
    world_data: DEFAULT_PROFILE_WORLD,
    player_data: DEFAULT_PROFILE_PLAYER
};

/// All the data saved to disk that represents a profile.
#[derive(Deserialize, Serialize)]
pub(super) struct ProfileData {
    pub(super) world_data: WorldData, //The data representing the world.
    pub(super) player_data: PlayerData //The data representing the player.
}