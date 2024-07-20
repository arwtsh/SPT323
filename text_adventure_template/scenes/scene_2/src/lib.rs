use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![ 
            "Scene2".to_string(),
            "scene2".to_string()],
        id: SceneId::Scene2
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene2)
}

pub struct Scene2;

impl Scene for Scene2 {
    fn enter_scene(&self) {
        println!("You walk along the path until you find a thick bog. 
        It looks like there's a dry path leading deeper into the bog to the LEFT.");
        println!("To the RIGHT is a path that goes the long way around the bog.");
    }
}