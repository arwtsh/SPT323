use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene3".to_string(),
            "scene3".to_string()],
        id: SceneId::Scene3
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene3)
}

pub struct Scene3;

impl Scene for Scene3 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("Moving along the shallow parts of the bog is slow going. 
        The mud pulls at your legs, making it hard to walk. Large bubbles slowly move up the viscous liquid until they burst with a loud BLUP. 
        When you reach the center of the bog, you hear a large roar as several tentacles burst from the murkey depths.");
        println!("To the LEFT is the creature and far away to the RIGHT is dry land.");
    }

    fn move_left(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene4));
    }

    fn move_right(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene5));
    }
}