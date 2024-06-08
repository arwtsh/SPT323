use std::collections::HashMap;

pub struct ItemManager {
    //Information about every item. This does not change or unload during gameplay.
    //This is mainly used to access items.
    //It is in a HashMap, meaning you can access the ItemData with just the ItemId enum.
    all_item_data : HashMap<item_id::Items, item_template::ItemData>,
    item_parses: HashMap<String, item_id::Items>
}

impl ItemManager {
    pub fn init() -> ItemManager {
        let manager = ItemManager {
            all_item_data: find_item_data(),
            item_parses: HashMap::new()
        };
        manager.compile_item_parses();
        manager
    }

    fn compile_item_parses(&self) {
        for item_data in self.all_item_data.iter() {
            self.
        }
    }
}

//Gets item data of every item and puts it in a hash map.
//The hash map makes it easily accessible.
fn find_item_data() -> HashMap<item_id::Items, item_template::ItemData> {
    let mut map = HashMap::new();
    map.insert(item_id::Items::Item1, item1::get_item_data());
    map
}

