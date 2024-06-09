use item_id::Items;

fn main() {
    let item_manager: item_manager::ItemManager = item_manager::ItemManager::init();

    let item: Items = item_manager.parse_item("item1".to_string());
    print!("Item: {}", item.to_string());
}
