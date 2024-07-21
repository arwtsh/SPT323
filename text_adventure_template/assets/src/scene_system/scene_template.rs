use crate::scene_system::scene_id::SceneId;

/// Data that is stored by the scene manager at game initialization.
/// It can be accessed even when the item isn't loaded.
pub struct SceneData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Scene.
    pub id: SceneId, //The Scene enum that is an ID for this scene.
}

/// Declares this as a scene that can be loaded
pub trait Scene {
    /// Invoked when this scene is unloaded from memory.
    fn unload(&self) {}
    /// Invoked when the player exits this scene.
    fn exit_scene(&self) {}
    /// Invoked when the player enters this scene.
    fn enter_scene(&self) {}
}