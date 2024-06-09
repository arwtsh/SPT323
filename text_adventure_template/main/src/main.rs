use item_id::Items;
use scene_id::Scenes;

fn main() {
    let item_manager: item_manager::ItemManager = item_manager::ItemManager::init();
    let scene_manager: scene_manager::SceneManager = scene_manager::SceneManager::init();

    let scene: Scenes = scene_manager.parse_scene("scene1".to_string());
    print!("Scene: {}", scene.to_string());
}

///Display to the terminal how to play the game and the commands to use.
pub fn print_help() {
    println!("You will be given a text description of a scene.");
    println!("Important or interactable objects in the scene will be CAPITALIZED.");
    println!("There are a list of commands you can type to interact with the scene.");
    println!("Commands are not case-sensitive. Here the are capitalized to stand out against their description.");
    println!("MOVE + ADJACENT LOCATION changes the which scene is being described. You can only move to locations mentioned in the scene.");
    println!("GRAB + LISTED ITEM adds an item listed in the scene into your inventory.");
    println!("INVENTORY lists all the items you have in your inventory.");
    println!("USE + ITEM IN INVENTORY + ON + OBJECT IN SCENE will (if correct) progress the scene and possibly remove the item from your inventory.");
    println!("EXAMINE + OBJECT IN SCENE will describe the specified object in more detail.");
    println!("INSPECT + ITEM IN INVENTORY will describe the item, potentially revealing hidden clues.");
    println!("LOOK will repeat the description of the location.");
    println!("HELP repeats these tips.");
    println!("EXIT closes the game.");
    println!("Occasionally a puzzle will have unique commands. The puzzle will list of those commands explicitly.");
}