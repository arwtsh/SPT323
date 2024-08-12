pub mod command_system {
    pub mod command_manager;
    pub mod command_schemes;
    pub mod commands;
}

pub mod event_system {
    pub mod event_manager;
    pub mod events;
    pub mod generated;
}

pub mod scene_system {
    pub mod scene_id;
    pub mod scene_loader;
    pub mod scene_manager;
    pub mod scene_template;
    pub mod static_scenes {
        pub mod main_menu;
        pub mod profile_select;
    }
}

pub mod save_system;

pub mod game_manager;
pub mod user_input;

use event_system::event_manager::{get_mut_event_system, get_event_system, EventSystem};
use event_system::events::EventType::OnApplicationShutdown;
use command_system::command_manager::get_command_manager;
use game_manager::get_game_manager;
use scene_system::scene_manager::get_scene_manager;
use save_system::save_system::get_save_system;

/// Initialize the game.
pub fn init_app() {
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
    get_scene_manager();
    get_save_system();
    get_event_system();
}

/// Set up the events in all the managers so they will be ready for the start of the game.
fn setup_events(event_system: &mut EventSystem) {
    scene_system::scene_manager::setup_events(event_system);
    game_manager::setup_events(event_system);
}