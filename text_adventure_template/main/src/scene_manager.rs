use std::collections::HashMap;
use scene_util::scene_id::Scenes;
use scene_util::scene_template::SceneData;
use crate::game_manager;

/// The singleton of the SceneManager
static mut INSTANCE: Option<SceneManager> = Option::None;

/// Get the SceneManager singleton as immutable.
pub fn get_scene_manager() -> &'static SceneManager {
    let scene_manager: &SceneManager;
    unsafe {
        //Initialize SceneManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(SceneManager::init());
        }
        scene_manager = INSTANCE.as_ref().unwrap();
    }
    scene_manager
}

/// Get the SceneManager singleton as mutable.
pub fn get_mut_scene_manager() -> &'static mut SceneManager {
    let scene_manager: &mut SceneManager;
    unsafe {
        //Initialize SceneManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(SceneManager::init());
        }
        scene_manager = INSTANCE.as_mut().unwrap();
    }
    scene_manager
}

pub struct SceneManager {
    //Information about every scene. This does not change or unload during gameplay.
    //This is mainly used to access scenes.
    //It is in a HashMap, meaning you can access the SceneData with just the SceneId enum.
    all_scene_data : HashMap<Scenes, SceneData>,
    //A map connecting strings to SceneIDs. This is used to parse user input.
    scene_parses: HashMap<String, Scenes>,
    //The current scene the player is in.
    current_scene: Scenes
}

impl SceneManager {
    ///Create a new instance of SceneManager.
    pub fn init() -> SceneManager {
        //Declare a new instance of SceneManager
        let mut manager = SceneManager {
            all_scene_data: find_scene_data(), //Find every SceneData and store it.
            scene_parses: HashMap::new(), //Initialize the scene_parse hash map.
            current_scene: Scenes::Scene1 //Set Scene1 as the starting scene.
        };
        manager.compile_scene_parses(); //Populate the scene_parse hash map.
        manager.move_scenes(manager.current_scene);
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

    fn get_scene_data(&self, scene: Scenes) -> &SceneData {
        //Get the result from the hash map. 
        //If the enum does not exist in the map, then the program should panic.
        let result = self.all_scene_data.get(&scene); 
        if result.is_some() {
            result.unwrap()
        } else {
            panic!("Scene {} does not have an entry in the all_scene_data hash map.", scene.to_string());
        }
    }

    //The player moved to the right.
    pub fn move_right(&mut self) {
        //Get the current scene data
        let current_scene_data = self.get_scene_data(self.current_scene);
        self.move_scenes(current_scene_data.right_scene);
    }

     //The player moved to the left.
     pub fn move_left(&mut self) {
        //Get the current scene data
        let current_scene_data = self.get_scene_data(self.current_scene);
        self.move_scenes(current_scene_data.left_scene);
    }

    fn move_scenes(&mut self, move_to: Scenes) {
        //Set new current scene
        self.current_scene = move_to;
        let new_scene_data = self.get_scene_data(self.current_scene);
        println!("{}", new_scene_data.description);
        if new_scene_data.left_scene == Scenes::None {
            game_manager::quit_game();
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
    map.insert(Scenes::Scene3, scene_3::get_scene_data());
    map.insert(Scenes::Scene4, scene_4::get_scene_data());
    map.insert(Scenes::Scene5, scene_5::get_scene_data());
    map.insert(Scenes::Scene6, scene_6::get_scene_data());
    map.insert(Scenes::Scene7, scene_7::get_scene_data());
    map.insert(Scenes::Scene8, scene_8::get_scene_data());
    map.insert(Scenes::Scene9, scene_9::get_scene_data());
    map.insert(Scenes::Scene10, scene_10::get_scene_data());
    map
}

