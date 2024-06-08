//Data that is stored by the item manager at game initialization.
//It can be accessed even when the item isn't loaded.
pub struct ItemData {
    pub identifiers: Vec<String>,
    pub id: item_id::Items
}