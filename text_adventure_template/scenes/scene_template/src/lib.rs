///Data that is stored by the item manager at game initialization.
///It can be accessed even when the item isn't loaded.
pub struct SceneData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Item.
    pub id: scene_id::Scenes //The Item enum that is an ID for this item.
}