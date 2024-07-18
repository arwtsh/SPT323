use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_event_system;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{
        identifiers: vec![
            "Scene4".to_string(),
            "scene4".to_string()],
        id: SceneId::Scene4
    }
}

pub struct Scene4;

impl Scene for Scene4 {
    fn enter_scene(&self) {
        println!("You charge the creature, hoping to make it past the tentacles. 
        The bog slows you down, and in your rush you fall over. The last thing you see is a large tencacle getting ready to strike.");
    }
}