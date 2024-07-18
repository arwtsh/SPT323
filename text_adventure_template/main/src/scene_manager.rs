use std::collections::HashMap;
use event_system::events_generated::EventDelegate::{OnGameStart, OnMoveScenesRequest};
use event_system::EventSystem;
use scene_util::{scene_id::SceneId, scene_template::Scene};
use scene_util::scene_template::SceneData;
use crate::{main_menu::MainMenu, scene_loader::SceneLoader};

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
    //Information about all dynamically loaded scenes. This does not change or unload during gameplay.
    //This is mainly used to access scenes.
    //It is in a HashMap, meaning you can access the SceneData with just the SceneId enum.
    all_scene_data : HashMap<SceneId, SceneData>,
    //A map connecting strings to SceneIDs. This is used to parse user input.
    scene_parses: HashMap<String, SceneId>,
    //The current scene the player is in.
    current_scene: SceneId,
    //The manager in charge of dynamically loading scenes.
    scene_loader: SceneLoader,
    //The main menu. This scene will always have an instance ready at all times.
    main_menu: MainMenu
}

impl SceneManager {
    ///Create a new instance of SceneManager.
    pub fn init() -> SceneManager {
        //Declare a new instance of SceneManager
        let mut manager = SceneManager {
            all_scene_data: HashMap::new(), //Find every SceneData and store it.
            scene_parses: HashMap::new(), //Initialize the scene_parse hash map.
            current_scene: SceneId::MainMenu, //The game will always initialize in the main menu.
            scene_loader: SceneLoader::new(),
            main_menu: MainMenu::new()
        };
        manager.all_scene_data = find_scene_data(&mut manager.scene_loader);
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
    pub fn parse_scene(&self, input: String) -> SceneId{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        let result = self.scene_parses.get(&input); 
        if result.is_some() {
            *result.unwrap()
        } else {
            SceneId::None
        }
    }

    fn get_scene_data(&self, scene: SceneId) -> &SceneData {
        //Get the result from the hash map. 
        //If the enum does not exist in the map, then the program should panic.
        let result = self.all_scene_data.get(&scene); 
        if result.is_some() {
            result.unwrap()
        } else {
            panic!("Scene {} does not have an entry in the all_scene_data hash map.", scene.to_string());
        }
    }
}

//Gets scene data of every scene and puts it in a hash map.
//The hash map makes it easily accessible.
fn find_scene_data(scene_loader: &mut SceneLoader) -> HashMap<SceneId, SceneData> {
    let mut map = HashMap::new();
    for scene_id in SceneId::iter() {
        map.insert(*scene_id, scene_loader.get_scene_data(*scene_id));
    }
    map
}

/// Moves what scene the application is using.
fn move_scenes(move_to: SceneId) {
    if move_to == SceneId::None {
        print!("Cannot move to scene \"None\" because it represents no scene.");
    }

    let scene_manager = get_mut_scene_manager();

    //Tell the current scene that it is exiting.
    if scene_manager.current_scene == SceneId::MainMenu {
        scene_manager.main_menu.exit_scene();
    }
    else {
        scene_manager.scene_loader.get_scene(scene_manager.current_scene).exit_scene();
    }


    //The the new scene that it is entering
    if move_to == SceneId::MainMenu {
        scene_manager.main_menu.enter_scene();
    }
    else {
        scene_manager.scene_loader.get_scene(move_to).enter_scene();
    }

    //Set new current scene
    scene_manager.current_scene = move_to;
}

/// Add listeners to all the events the scene manager will use.
pub fn setup_events(event_system: &mut EventSystem) {
    //When the game starts, make the scene the main menu.
    event_system.add_listener(OnGameStart(|| {
        get_scene_manager().main_menu.enter_scene();
    }));

    event_system.add_listener(OnMoveScenesRequest(move_scenes));
}