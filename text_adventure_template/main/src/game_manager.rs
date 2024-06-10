use item_manager::ItemManager;
use scene_manager::SceneManager;
use crate::command_manager::CommandManager;

///A container for every manager that can be used by commands. 
//Every command will accept a Managers reference as a simple dependency injection pattern.
pub struct Managers {
    item_manager: ItemManager,
    scene_manager: SceneManager,
    command_manager: CommandManager
}

impl Managers {
    //Getters for private managers. 
    //They return references and not copies of the managers.
    pub fn get_item_manager(&self) -> &ItemManager {
        &self.item_manager
    }
    pub fn get_scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }
    pub fn get_command_manager(&self) -> &CommandManager {
        &self.command_manager
    }
}

///Holds the base logic for all other managers in the game, as well as the main game loop.
pub struct GameManager {
    ///A container for every manager in the game.
    managers: Managers
}

impl GameManager {
    ///Create a new GameManager
    pub fn new() -> GameManager{
        GameManager {
            managers: Managers {
                item_manager: ItemManager::init(),
                scene_manager: SceneManager::init(),
                command_manager: CommandManager::init()
            }
        }
    }

    ///Starts the game.
    pub fn start_game(&self) {
        self.managers.command_manager.parse_user_input("left".to_string(), &self.managers);
    }
}