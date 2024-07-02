use crate::command_manager::parse_user_input;

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

    //Infinite loop
    loop {
        user_input.clear(); //Clear previous user input.
        std::io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Read user input from terminal.
        parse_user_input(&user_input.trim().to_string()); //Interpret the player input.
    }
}

/// Exit the game
pub fn quit_game() {
    std::process::exit(0); //Exit the game
}