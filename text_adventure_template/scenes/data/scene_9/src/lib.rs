use scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene9".to_string(),
            "scene9".to_string()],
        id: scene_id::Scenes::Scene9,  //Set the id for this scene.
        left_scene: scene_id::Scenes::None, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::None, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "You found yourself lost in the forest. Without food, you eventually starve. GAME OVER.".to_string()
    }
}