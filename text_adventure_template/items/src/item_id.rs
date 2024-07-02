#[derive(PartialEq, Eq, Hash, Clone, Copy)]
///An ID for items.
pub enum Items {
    None, //A representation of an error, no item.
    Item1,
    Item2,
    Item3,
    Item4,
    Item5
}

impl Items {
    pub fn to_string(&self) -> &str {
        match *self {
            Items::None => "None",
            Items::Item1 => "Item1",
            Items::Item2 => "Item2",
            Items::Item3 => "Item3",
            Items::Item4 => "Item4",
            Items::Item5 => "Item5"
        }
    }
}