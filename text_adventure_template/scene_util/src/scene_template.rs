use crate::scene_id;

///Data that is stored by the scene manager at game initialization.
///It can be accessed even when the item isn't loaded.
pub struct SceneData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Scene.
    pub id: scene_id::Scenes, //The Scene enum that is an ID for this scene.
    pub left_scene: scene_id::Scenes, //The scene that the player goes to when moving left.
    pub right_scene: scene_id::Scenes, //The scene that the player goes to when moving right.
    pub description: String //What is printed to the screen when the player moves to this scene.
}