use scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene1".to_string(),
            "scene1".to_string()
        ],
        id: scene_id::Scenes::Scene1,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene2, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene6, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "The Dark Forest stands before you. You need to go through it to get to Grandma's house. 
        The forest is notorious for disappearances. It's easy to get lost in it's vast labyranth. 
        Entering the forest, the path immediatly branches. 
        \nTo the LEFT is the sound of bubbling water. To the RIGHT you can barely make out small points of light.".to_string()
    }
}

