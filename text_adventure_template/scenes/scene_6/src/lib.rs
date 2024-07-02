use scene_util::scene_id;
use scene_util::scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene6".to_string(),
            "scene6".to_string()],
        id: scene_id::Scenes::Scene6,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene7, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene8, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "You continue along the dark path. You notice two points of light that constantly stay some distance away.
        After some time, they start to get closer. Eventually, a large worlf with beady glowing eyes stands above you at twice your height.
        The wolf then speaks to you in a low breathy voice. \"You seem lost. I can help you if you follow me.\"
        The wolf starts to head to the RIGHT. Do you follow or run away to the LEFT?".to_string()
    }
}