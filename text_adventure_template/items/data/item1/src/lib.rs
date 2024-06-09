use item_template::ItemData;

///Get the Item_Data for this item.
pub fn get_item_data() -> ItemData {
    ItemData{ //Create new item.
        identifiers: vec![ //populate the identifiers with string literals. These will be what is used to match player input this item.
            "item1".to_string()],
        id: item_id::Items::Item1 //Set the id for this item.
    }
}