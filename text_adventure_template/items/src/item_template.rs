use crate::item_id;

///Data that is stored by the item manager at game initialization.
///It can be accessed even when the item isn't loaded.
pub struct ItemData {
    pub identifiers: Vec<String>, //All strings that could be referencing this Item.
    pub id: item_id::Items //The Item enum that is an ID for this item.
}