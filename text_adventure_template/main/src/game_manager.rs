use item_manager::ItemManager;
use scene_manager::SceneManager;
use crate::command_manager::CommandManager;

///A container for every manager that can be used by commands. 
//Every command will accept a Managers reference as a simple dependency injection pattern.
pub struct Managers {
    item_manager: ItemManager,
    scene_manager: SceneManager,
}

impl Managers {
    //Getters for private managers. 
    //They return references and not copies of the managers.
    pub fn get_item_manager(&self) -> &ItemManager {
        &self.item_manager
    }
    pub fn get_scene_manager(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }
}

///Holds the base logic for all other managers in the game, as well as the main game loop.
pub struct GameManager {
    ///A container for every manager in the game.
    managers: Managers,
    //The CommandManager is not included with the managers because it will cause mutable reference borrowing errors.
    command_manager: CommandManager
}

impl GameManager {
    ///Create a new GameManager
    pub fn new() -> GameManager{
        GameManager {
            managers: Managers {
                item_manager: ItemManager::init(),
                scene_manager: SceneManager::init(),
            },
            command_manager: CommandManager::init()
        }
    }

    ///Starts the game.
    pub fn start_game(&mut self) {
        //Start the main game loop
        self.game_loop();
    }

    ///The main game loop.
    fn game_loop(&mut self) {
        //Declare input buffer
        let mut user_input = String::new();

        //Infinite loop
        loop {
            user_input.clear(); //Clear previous user input.
            std::io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Read user input from terminal.
            self.command_manager.parse_user_input(&user_input.trim().to_string(), &mut self.managers); //Interpret the player input.
        }
    }

}