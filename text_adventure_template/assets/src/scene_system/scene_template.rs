use crate::{event_system::event_manager::EventSystem, scene_system::scene_id::SceneId};

/// Data that is stored by the scene manager at game initialization.
/// It can be accessed even when the item isn't loaded.
pub struct SceneData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Scene.
    pub id: SceneId, //The Scene enum that is an ID for this scene.
}

/// Declares this as a scene that can be loaded
pub trait Scene {
    /// Invoked when this scene is unloaded from memory.
    fn unload(&self, _event_system: &mut EventSystem) {}
    /// Invoked when the player exits this scene.
    fn exit_scene(&self, _event_system: &mut EventSystem) {}
    /// Invoked when the player enters this scene.
    fn enter_scene(&self, _event_system: &mut EventSystem) {}
    /// Invoked when the command move_left is called.
    fn move_left(&self, _event_system: &mut EventSystem) {}
    /// Invoked when the command move_right is called.
    fn move_right(&self, _event_system: &mut EventSystem) {}
}

//
// STATIC VARIABLES ARE NOT SHARED WITH DYNAMIC LIBRARIES!!!
//