use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene7".to_string(),
            "scene7".to_string()
        ],
        id: SceneId::Scene7
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene7)
}

pub struct Scene7;

impl Scene for Scene7 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("As you run away from the wolf, it howls in anger and starts to chase after you.");
        println!("It is significantly faster than you. A branch in the path is ahead of you. Do you head RIGHT or LEFT?");
    }

    fn move_left(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene8));
    }

    fn move_right(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene10));
    }
}