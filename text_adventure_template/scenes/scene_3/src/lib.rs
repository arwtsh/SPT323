use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene3".to_string(),
            "scene3".to_string()],
        id: SceneId::Scene3
    }
}

pub struct Scene3;

impl Scene for Scene3 {
    fn enter_scene(&self) {
        println!("Moving along the shallow parts of the bog is slow going. 
        The mud pulls at your legs, making it hard to walk. Large bubbles slowly move up the viscous liquid until they burst with a loud BLUP. 
        When you reach the center of the bog, you hear a large roar as several tentacles burst from the murkey depths.");
        println!("To the LEFT is the creature and far away to the RIGHT is dry land.");
    }
}