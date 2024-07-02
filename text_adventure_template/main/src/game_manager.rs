use crate::command_manager::parse_user_input;
use crate::scene_manager;

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
            is_game_active: true
        }
    }
}

/// Starts the game.
pub fn start_game() {
    get_mut_game_manager().is_game_active = true;

    //Start the game loop
    game_loop();
}

/// The main game loop.
fn game_loop() {
    //Declare input buffer
    let mut user_input = String::new();

    //Loop while the game is active.
    while get_game_manager().is_game_active {
        user_input.clear(); //Clear previous user input.
        std::io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Read user input from terminal.
        parse_user_input(&user_input.trim().to_string()); //Interpret the player input.
    }

    //Do code logic that occurs when the game is exiting.
    end_loop();
}

/// Tells the game loop to exit the game at the end of the next loop.
/// If an immediate exit is desired, std::process::exit(0); should be used instead.
pub fn quit_game() {
    //Tell the game manager the game is no longer active.
    get_mut_game_manager().is_game_active = false;
}

/// Code that executes when the game is attempting to quit
fn end_loop() {
    //Ask the user if they want to play again.
    println!("Do you want to play again? (y/n)");

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Read user input from terminal.
    if ["y", "Y", "yes", "Yes", "YES"].contains(&user_input.trim()){
        restart_game(); //Start the game over.
    }
    else {
        game_shutdown();
    }
}

/// Restarts the game
fn restart_game() {
    scene_manager::get_mut_scene_manager().reset(); //Move the player to the first location again.

    start_game(); //Start the game loop again.
}

/// Code that is ran when the game shuts down
fn game_shutdown() {
    //Game should be automatically exiting normally
    //We could call std::process::exit(0); to force the game to close, though.
    print!("Thanks for playing!");
}