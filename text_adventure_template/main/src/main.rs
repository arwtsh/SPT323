use command_manager::get_command_manager;
use game_manager::get_game_manager;
use item_manager::get_item_manager;
use scene_manager::get_scene_manager;
use items::item_manager;

mod game_manager;
mod command_manager;
mod scene_manager;

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
    load_start();
    game_manager::start_game();
}

/// Load all the necessary items into memory at the start of the application.
fn load_start() {
    //Call each of the managers once so that they generate at the start of the application. 
    //This isn't necessary entirely necessary, since they will automatically load the first time their needed, but it makes sence happenng here.
    get_command_manager();
    get_game_manager();
    get_item_manager();
    get_scene_manager();
}