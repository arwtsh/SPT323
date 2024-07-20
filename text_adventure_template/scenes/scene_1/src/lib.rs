use scene_util::scene_id::SceneId;
use scene_util::scene_template::{SceneData, Scene};
use event_system::get_mut_event_system;

/// Get the Scene_Data for this scene.
#[no_mangle]
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![
            "Scene1".to_string(),
            "scene1".to_string()
        ],
        id: SceneId::Scene1
    }
}

/// Get the scene for this library.
#[no_mangle]
pub fn get_scene() -> Box<dyn Scene> {
    Box::new(Scene1)
}

pub struct Scene1;

impl Scene for Scene1 {
    fn enter_scene(&self) {
        println!("The Dark Forest stands before you. You need to go through it to get to Grandma's house. 
        The forest is notorious for disappearances. It's easy to get lost in it's vast labyranth. 
        Entering the forest, the path immediatly branches.");
        println!("To the LEFT is the sound of bubbling water. To the RIGHT you can barely make out small points of light.");

        //get_mut_command_manager().active_commands_scheme = CommandSchemes::Gameplay;
    }
}
