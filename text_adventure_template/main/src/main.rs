use game_manager::GameManager;

mod game_manager;
mod command_manager;

//
// ADD TO THE MODS BELOW WHEN ADDING NEW COMMANDS
//
pub mod commands {
    pub mod command_exit;
    pub mod command_help;
    pub mod command_left;
    pub mod command_right;
}

fn main() {
    let mut game_manager: GameManager = GameManager::new();
    game_manager.start_game();
}