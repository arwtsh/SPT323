use scene_util::scene_id;
use scene_util::scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene3".to_string(),
            "scene3".to_string()],
        id: scene_id::Scenes::Scene3,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene4, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene5, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "Moving along the shallow parts of the bog is slow going. 
        The mud pulls at your legs, making it hard to walk. Large bubbles slowly move up the viscous liquid until they burst with a loud BLUP. 
        When you reach the center of the bog, you hear a large roar as several tentacles burst from the murkey depths.
        To the LEFT is the creature and far away to the RIGHT is dry land.".to_string()
    }
}