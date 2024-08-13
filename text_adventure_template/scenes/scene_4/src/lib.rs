use assets::event_system::event_manager::EventSystem;
use assets::event_system::events::EventType;
use assets::scene_system::scene_id::SceneId;
use assets::scene_system::scene_template::{SceneData, Scene};

///Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene4".to_string(),
            "scene4".to_string()],
        id: SceneId::Scene4
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene4)
}

pub struct Scene4;

impl Scene for Scene4 {
    fn enter_scene(&self, _event_system: &mut EventSystem) {
        println!("You charge the creature, hoping to make it past the tentacles. 
        The bog slows you down, and in your rush you fall over. The last thing you see is a large tencacle getting ready to strike.");
        println!("GAME OVER");

        //Return to the main menu
        _event_system.invoke(EventType::LoseGame);
    }
}