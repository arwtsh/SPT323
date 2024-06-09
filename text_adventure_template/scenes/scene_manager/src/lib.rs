use std::collections::HashMap;
use scene_id::Scenes;
use scene_template::SceneData;

pub struct SceneManager {
    //Information about every scene. This does not change or unload during gameplay.
    //This is mainly used to access scenes.
    //It is in a HashMap, meaning you can access the SceneData with just the SceneId enum.
    all_scene_data : HashMap<Scenes, SceneData>,
    //A map connecting strings to SceneIDs. This is used to parse user input.
    scene_parses: HashMap<String, Scenes>
}

impl SceneManager {
    ///Create a new instance of SceneManager.
    pub fn init() -> SceneManager {
        //Declare a new instance of SceneManager
        let mut manager = SceneManager {
            all_scene_data: find_scene_data(), //Find every SceneData and store it.
            scene_parses: HashMap::new() //Initialize the scene_parse hash map.
        };
        manager.compile_scene_parses(); //Populate the scene_parse hash map.
        manager //Return the new instance.
    }

    ///Compiles the HashMap to have all string-id pairs from scene data.
    fn compile_scene_parses(&mut self) {
        //Loop through every scene data.
        for scene_data in self.all_scene_data.iter() {
            //Loop through the collection of strings in an scene data.
            for scene_identifier in &scene_data.1.identifiers {
                if self.scene_parses.contains_key(scene_identifier) {
                    panic!("An scene parse identifier \"{}\" in scene {} already points to scene {}", scene_identifier, scene_data.0.to_string(), self.scene_parses.get(scene_identifier).unwrap().to_string());
                }
                else {
                    //Populate the scene_parse hash map from the data in scene data.
                    self.scene_parses.insert(scene_identifier.clone(), *scene_data.0);
                }
            }
        }
    }

    ///Parse a string to scene id
    pub fn parse_scene(&self, input: String) -> Scenes{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        let result = self.scene_parses.get(&input); 
        if result.is_some() {
            *result.unwrap()
        } else {
            Scenes::None
        }
    }
}

//
// MODIFY THE BELOW FUNCTIONS WHEN ADDING NEW SCENES
//

//Gets scene data of every scene and puts it in a hash map.
//The hash map makes it easily accessible.
fn find_scene_data() -> HashMap<Scenes, SceneData> {
    let mut map = HashMap::new();
    map.insert(Scenes::Scene1, scene_1::get_scene_data());
    map.insert(Scenes::Scene2, scene_2::get_scene_data());
    map
}

