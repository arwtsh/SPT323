use std::collections::HashMap;
use crate::item_id::Items;
use crate::item_template::ItemData;
use crate::data;

/// The singleton of the ItemManager
static mut INSTANCE: Option<ItemManager> = Option::None;

/// Get the ItemManager singleton as immutable.
pub fn get_item_manager() -> &'static ItemManager {
    let item_manager: &ItemManager;
    unsafe {
        //Initialize ItemManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(ItemManager::init());
        }
        item_manager = INSTANCE.as_ref().unwrap();
    }
    item_manager
}

/// Get the ItemManager singleton as mutable.
pub fn get_mut_item_manager() -> &'static mut ItemManager {
    let item_manager: &mut ItemManager;
    unsafe {
        //Initialize ItemManager if it hasn't been already.
        if INSTANCE.is_none() {
            INSTANCE = Option::Some(ItemManager::init());
        }
        item_manager = INSTANCE.as_mut().unwrap();
    }
    item_manager
}

pub struct ItemManager {
    //Information about every item. This does not change or unload during gameplay.
    //This is mainly used to access items.
    //It is in a HashMap, meaning you can access the ItemData with just the ItemId enum.
    all_item_data : HashMap<Items, ItemData>,
    //A map connecting strings to ItemIDs. This is used to parse user input.
    item_parses: HashMap<String, Items>
}

impl ItemManager {
    ///Create a new instance of ItemManager.
    pub fn init() -> ItemManager {
        //Declare a new instance of ItemManager
        let mut manager = ItemManager {
            all_item_data: find_item_data(), //Find every ItemData and store it.
            item_parses: HashMap::new() //Initialize the item_parse hash map.
        };
        manager.compile_item_parses(); //Populate the item_parse hash map.
        manager //Return the new instance.
    }

    ///Compiles the HashMap to have all string-id pairs from item data.
    fn compile_item_parses(&mut self) {
        //Loop through every item data.
        for item_data in self.all_item_data.iter() {
            //Loop through the collection of strings in an item data.
            for item_identifier in &item_data.1.identifiers {
                if self.item_parses.contains_key(item_identifier) {
                    panic!("An item parse identifier \"{}\" in item {} already points to item {}", item_identifier, item_data.0.to_string(), self.item_parses.get(item_identifier).unwrap().to_string());
                }
                else {
                    //Populate the item_parse hash map from the data in item data.
                    self.item_parses.insert(item_identifier.clone(), *item_data.0);
                }
            }
        }
    }

    ///Parse a string to item id
    pub fn parse_item(&self, input: String) -> Items{
        //Get the result from the hash map. 
        //If the string does not exist in the map, it returns Option.none
        let result = self.item_parses.get(&input); 
        if result.is_some() {
            *result.unwrap()
        } else {
            Items::None
        }
    }
}

//
// MODIFY THE BELOW FUNCTIONS WHEN ADDING NEW ITEMS
//

//Gets item data of every item and puts it in a hash map.
//The hash map makes it easily accessible.
fn find_item_data() -> HashMap<Items, ItemData> {
    let mut map = HashMap::new();
    map.insert(Items::Item1, data::item1::item1::get_item_data());
    map.insert(Items::Item2, data::item2::item2::get_item_data());
    map
}

