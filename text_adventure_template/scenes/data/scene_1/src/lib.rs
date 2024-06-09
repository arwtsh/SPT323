use scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene1".to_string(),
            "scene1".to_string()],
        id: scene_id::Scenes::Scene1  //Set the id for this scene.
    }
}