use crate::item_template::ItemData;
use crate::item_id::Items;

///Get the Item_Data for this item.
pub fn get_item_data() -> ItemData {
    ItemData{ //Create new item.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this item.
            "item2".to_string(),
            "Item2".to_string()],
        id: Items::Item2 //Set the id for this item.
    }
}