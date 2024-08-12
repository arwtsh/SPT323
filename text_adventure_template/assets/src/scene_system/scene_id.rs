use std::slice::Iter;

use serde::{Deserialize, Serialize};

use super::{scene_template::Scene, static_scenes::{main_menu::MainMenu, profile_select::ProfileSelect}};

/// The starting scene for a fresh save.
/// This should not be the main menu.
pub const STARTING_SCENE: SceneId = SceneId::Scene1;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
/// An ID for items.
pub enum SceneId {
    None, //A representation of an error, no scene.
    MainMenu, //A constant scene that is handled directly by scene_manager and not scene_loader
    ProfileSelect, //A scene that represents a sub-menu of the main menu.
    Scene1,
    Scene2,
    Scene3,
    Scene4,
    Scene5,
    Scene6,
    Scene7,
    Scene8,
    Scene9,
    Scene10
}

impl SceneId {
    pub fn to_string(&self) -> &str {
        match *self {
            SceneId::None => "None",
            SceneId::MainMenu => "MainMenu",
            SceneId::ProfileSelect => "ProfileSelect",
            SceneId::Scene1 => "Scene1",
            SceneId::Scene2 => "Scene2",
            SceneId::Scene3 => "Scene3",
            SceneId::Scene4 => "Scene4",
            SceneId::Scene5 => "Scene5",
            SceneId::Scene6 => "Scene6",
            SceneId::Scene7 => "Scene7",
            SceneId::Scene8 => "Scene8",
            SceneId::Scene9 => "Scene9",
            SceneId::Scene10 => "Scene10"
        }
    }

    /// Get the static scene reference associated with this ID.
    /// If the ID represents a dynamic scene and not a static one, the value 'Option::None' is passed instead.
    pub fn get_static_scene(&self) -> Option<Box<dyn Scene>> {
        match *self {
            SceneId::MainMenu => Some(Box::new(MainMenu)),
            SceneId::ProfileSelect => Some(Box::new(ProfileSelect)),
            _ => None
        }
    }

    /// Get the name of the library crate that houses this scene.
    /// This is used for the dynamic library loading.
    /// If something is misspelled in here, the library will not load and the game will crash.
    /// This should match the name field inside the library's toml.
    pub fn get_lib(&self) -> &str {
        match *self {
            SceneId::None => "",
            SceneId::MainMenu => "",
            SceneId::ProfileSelect => "",
            SceneId::Scene1 => "scene_1",
            SceneId::Scene2 => "scene_2",
            SceneId::Scene3 => "scene_3",
            SceneId::Scene4 => "scene_4",
            SceneId::Scene5 => "scene_5",
            SceneId::Scene6 => "scene_6",
            SceneId::Scene7 => "scene_7",
            SceneId::Scene8 => "scene_8",
            SceneId::Scene9 => "scene_9",
            SceneId::Scene10 => "scene_10"
        }
    }

    pub fn iter() -> Iter<'static, SceneId> {
        static SCENES: [SceneId; 10] = [
            SceneId::Scene1, 
            SceneId::Scene2,
            SceneId::Scene3,
            SceneId::Scene4,
            SceneId::Scene5,
            SceneId::Scene6,
            SceneId::Scene7,
            SceneId::Scene8,
            SceneId::Scene9,
            SceneId::Scene10
        ];
        SCENES.iter()
    }

    /// Tell if a scene should be saved to profile data.
    /// Such scenes should only be gameplay, not things like Main Menu
    pub fn is_saveable(&self) -> bool{
        match *self {
            SceneId::None => false,
            SceneId::MainMenu => false,
            SceneId::ProfileSelect => false,
            _ => true
        }
    }
}