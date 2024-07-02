#[derive(PartialEq, Eq, Hash, Clone, Copy)]
///An ID for items.
pub enum Scenes {
    None, //A representation of an error, no scene.
    Scene1,
    Scene2,
    Scene3,
    Scene4,
    Scene5,
    Scene6,
    Scene7,
    Scene8,
    Scene9,
    Scene10
}

impl Scenes {
    pub fn to_string(&self) -> &str {
        match *self {
            Scenes::None => "None",
            Scenes::Scene1 => "Scene1",
            Scenes::Scene2 => "Scene2",
            Scenes::Scene3 => "Scene3",
            Scenes::Scene4 => "Scene4",
            Scenes::Scene5 => "Scene5",
            Scenes::Scene6 => "Scene6",
            Scenes::Scene7 => "Scene7",
            Scenes::Scene8 => "Scene8",
            Scenes::Scene9 => "Scene9",
            Scenes::Scene10 => "Scene10"
        }
    }
}