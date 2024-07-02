use scene_util::scene_id;
use scene_util::scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene7".to_string(),
            "scene7".to_string()],
        id: scene_id::Scenes::Scene7,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene8, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene10, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "As you run away from the wolf, it howls in anger and starts to chase after you. 
        It is significantly faster than you. A branch in the path is ahead of you. Do you head RIGHT or LEFT?".to_string()
    }
}