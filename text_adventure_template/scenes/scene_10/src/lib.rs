use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene10".to_string(),
            "scene10".to_string()
        ],
        id: SceneId::Scene10
    }
}

pub struct Scene10;

impl Scene for Scene10 {
    fn enter_scene(&self) {
        println!("You see the end of the forest line, and Grandma's house in the distance.");
    }
}