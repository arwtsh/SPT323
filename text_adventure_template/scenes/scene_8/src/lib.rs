use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene8".to_string(),
            "scene8".to_string()
        ],
        id: SceneId::Scene8
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene8)
}

pub struct Scene8;

impl Scene for Scene8 {
    fn enter_scene(&self) {
        println!("The wolf corners you, and with a snarl says: \"I rarely get to eat something as large as you. I will feast well tonight.");
    }
}