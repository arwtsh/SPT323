use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene5".to_string(),
            "scene5".to_string()
        ],
        id: SceneId::Scene5
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene5)
}

pub struct Scene5;

impl Scene for Scene5 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("You are on the long way around the bog. It takes a while, and the path keeps turning to the left. 
        You're not sure when to turn off the path deeper into the forest.");
        println!("Do you continue LEFT around the bog or turn RIGHT back into the forest?");
    }

    fn move_left(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene9));
    }

    fn move_right(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene10));
    }
}