use command_manager::get_command_manager;
use game_manager::get_game_manager;
use item_manager::get_item_manager;
use scene_manager::get_scene_manager;
use items::item_manager;
use save_system::{self, get_save_system};
use event_system::{get_event_system, get_mut_event_system, EventSystem};
use event_system::events::EventType::OnApplicationShutdown;

mod game_manager;
mod command_manager;
mod scene_manager;
mod scene_loader;

mod main_menu;

//
// ADD TO THE MODS BELOW WHEN ADDING NEW COMMANDS
//
pub mod commands {
    pub mod command_schemes;
    pub mod command_exit;
    pub mod command_help;
    pub mod command_left;
    pub mod command_right;
    pub mod command_credits;
    pub mod command_play;
    pub mod command_profile;
}

fn main() {
    load_start();
    setup_events(get_mut_event_system());
    game_manager::start_game();

    //At the end of the program's lifetime, have a final event get thrown.
    get_event_system().invoke(OnApplicationShutdown)
}

/// Load all the necessary items into memory at the start of the application.
fn load_start() {
    //Call each of the managers once so that they generate at the start of the application. 
    //This isn't necessary entirely necessary, since they will automatically load the first time their needed, but it makes sence happenng here.
    get_command_manager();
    get_game_manager();
    get_item_manager();
    get_scene_manager();
    get_save_system();
    get_event_system();
}

/// Set up the events in all the managers so they will be ready for the start of the game.
fn setup_events(event_system: &mut EventSystem) {
    scene_manager::setup_events(event_system);
    game_manager::setup_events(event_system);
}