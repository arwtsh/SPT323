use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene10".to_string(),
            "scene10".to_string()
        ],
        id: SceneId::Scene10
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene10)
}

pub struct Scene10;

impl Scene for Scene10 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("You see the end of the forest line, and Grandma's house in the distance.");
        println!("GAME OVER");

        _event_system.invoke(EventType::WinGame);
    }
}