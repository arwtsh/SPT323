use crate::event_system::event_manager::{get_event_system, EventSystem};
use crate::event_system::events::EventType::{OnGameStart, OnMoveScenesRequest};
use crate::event_system::generated::EventDelegate::{OnApplicationShutdown, QuitApplication, WinGame, LoseGame};
use crate::scene_system::scene_id::SceneId::MainMenu;
use crate::command_system::command_manager::parse_user_input;
use crate::save_system::save_system::get_mut_save_system;
use crate::scene_system::scene_id::STARTING_SCENE;
use crate::user_input;

/// The singleton of the GameManager
static mut INSTANCE: Option<GameManager> = Option::None;

/// Get the GameManager singleton as immutable.
pub fn get_game_manager() -> &'static GameManager {
    let game_manager: &GameManager;
    unsafe {
        //Initialize GameManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(GameManager::init());
        }
        game_manager = INSTANCE.as_ref().unwrap();
    }
    game_manager
}

/// Get the GameManager singleton as mutable.
pub fn get_mut_game_manager() -> &'static mut GameManager {
    let game_manager: &mut GameManager;
    unsafe {
        //Initialize GameManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(GameManager::init());
        }
        game_manager = INSTANCE.as_mut().unwrap();
    }
    game_manager
}

/// Holds the base logic for all other managers in the game, as well as the main game loop.
pub struct GameManager {
    /// A boolean that tells if the game is currently active. 
    /// It should not be set outside of the game_manager file
    pub is_game_active: bool
}

impl GameManager {
    /// Create a new GameManager
    pub fn init() -> GameManager{
        GameManager {
            is_game_active: false
        }
    }
}

/// Starts the game.
pub fn start_game() {
    get_mut_game_manager().is_game_active = true;

    get_event_system().invoke(OnGameStart);

    game_loop();
}

/// The main game loop.
fn game_loop() {
    //Loop while the game is active.
    while get_game_manager().is_game_active {
        parse_user_input(&user_input::get_user_input().trim().to_string()); //Interpret the player input.
    }
}

/// Code that is ran when the game shuts down
/// This code does not run if std::process::exit(0); is called directly.
fn on_game_shutdown() {
    //Game should be automatically exiting normally
    //We could call  to force the game to close, though.
    print!("Thanks for playing!");
}

/// Tells the game loop to exit the game at the end of the next loop.
/// If an immediate exit (crash) is desired, std::process::exit(0); should be used instead.
fn quit_game() {
    //Tell the game manager the game is no longer active.
    get_mut_game_manager().is_game_active = false;
}

/// Logic that happens when the player wins the game.
fn win_game() {
    println!("You win!");
    //Reset the profile so the player can play again.
    get_mut_save_system().set_current_scene(STARTING_SCENE);
    //Move the player to the main menu
    get_event_system().invoke(OnMoveScenesRequest(MainMenu));
}

/// Logic that happens when the player wins the game.
fn lose_game() {
    println!("You lose, try again!");
    //Reset the profile so the player can play again.
    get_mut_save_system().set_current_scene(STARTING_SCENE);
    //Move the player to the main menu
    get_event_system().invoke(OnMoveScenesRequest(MainMenu));
}

pub fn setup_events(event_system: &mut EventSystem) {
    //Have code execute when the application shuts down.
    event_system.add_listener(OnApplicationShutdown(on_game_shutdown));
    event_system.add_listener(QuitApplication(quit_game));
    event_system.add_listener(WinGame(win_game));
    event_system.add_listener(LoseGame(lose_game));
}