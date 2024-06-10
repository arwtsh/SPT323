use scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene5".to_string(),
            "scene5".to_string()],
        id: scene_id::Scenes::Scene5,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene9, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene10, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "You are on the long way around the bog. It takes a while, and the path keeps turning to the left. 
        You're not sure when to turn off the path deeper into the forest. 
        Do you continue LEFT around the bog or turn RIGHT back into the forest?".to_string()
    }
}