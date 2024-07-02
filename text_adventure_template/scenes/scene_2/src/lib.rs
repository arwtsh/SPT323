use scene_util::scene_id;
use scene_util::scene_template::SceneData;

///Get the Scene_Data for this scene.
pub fn get_scene_data() -> SceneData {
    SceneData{ //Create new scene.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this scene.
            "Scene2".to_string(),
            "scene2".to_string()],
        id: scene_id::Scenes::Scene2,  //Set the id for this scene.
        left_scene: scene_id::Scenes::Scene3, //Set the id for the scene the player moves to when going left.
        right_scene: scene_id::Scenes::Scene5, //Set the id for the scene the player moves to when going right.
        //The text that is printed to the screen when entering this scene.
        description: "You walk along the path until you find a thick bog. 
        It looks like there's a dry path leading deeper into the bog to the LEFT. 
        To the RIGHT is a path that goes the long way around the bog.".to_string()
    }
}