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

    /// Get the name of the library crate that houses this scene.
    /// This is used for the dynamic library loading.
    /// If something is misspelled in here, the library will not load and the game will crash.
    /// This should match the name field inside the library's toml.
    pub fn get_lib(&self) -> &str {
        match *self {
            Scenes::None => "",
            Scenes::Scene1 => "scene_1",
            Scenes::Scene2 => "scene_2",
            Scenes::Scene3 => "scene_3",
            Scenes::Scene4 => "scene_4",
            Scenes::Scene5 => "scene_5",
            Scenes::Scene6 => "scene_6",
            Scenes::Scene7 => "scene_7",
            Scenes::Scene8 => "scene_8",
            Scenes::Scene9 => "scene_9",
            Scenes::Scene10 => "scene_10"
        }
    }
}