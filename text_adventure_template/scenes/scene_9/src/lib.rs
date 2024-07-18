use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene9".to_string(),
            "scene9".to_string()
        ],
        id: SceneId::Scene9
    }
}

pub struct Scene9;

impl Scene for Scene9 {
    fn enter_scene(&self) {
        println!("You found yourself lost in the forest. Without food, you eventually starve.");
    }
}