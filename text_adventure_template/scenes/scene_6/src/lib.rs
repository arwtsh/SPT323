use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene6".to_string(),
            "scene6".to_string()
        ],
        id: SceneId::Scene6
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene6)
}

pub struct Scene6;

impl Scene for Scene6 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("You continue along the dark path. You notice two points of light that constantly stay some distance away.
        After some time, they start to get closer. Eventually, a large worlf with beady glowing eyes stands above you at twice your height.
        The wolf then speaks to you in a low breathy voice. \"You seem lost. I can help you if you follow me.");
        println!("The wolf starts to head to the RIGHT. Do you follow or run away to the LEFT?");
    }

    fn move_left(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene7));
    }

    fn move_right(&self, _event_system: &mut EventSystem) {
        _event_system.invoke(EventType::OnMoveScenesRequest(SceneId::Scene8));
    }
}