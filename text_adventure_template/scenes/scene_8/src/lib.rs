use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene8".to_string(),
            "scene8".to_string()
        ],
        id: SceneId::Scene8
    }
}

pub struct Scene8;

impl Scene for Scene8 {
    fn enter_scene(&self) {
        println!("The wolf corners you, and with a snarl says: \"I rarely get to eat something as large as you. I will feast well tonight.");
    }
}