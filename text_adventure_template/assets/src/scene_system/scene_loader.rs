use libloading::{Library, library_filename, Symbol};
use crate::{event_system::event_manager::get_mut_event_system, scene_system::{scene_id::SceneId, scene_template::{Scene, SceneData}}};
use std::collections::HashMap;


pub struct SceneLoader {
    //All of the currently loaded scene libraries.
    loaded_libraries: HashMap<SceneId, Library>,
    //All of the cached scenes
    loaded_scenes: HashMap<SceneId, Box<dyn Scene>>
}

impl SceneLoader {
    pub fn new() -> Self {
        SceneLoader {
            loaded_libraries: HashMap::new(),
            loaded_scenes: HashMap::new()
        }
    }

    /// Get the scene library.
    /// Will load the scene if it isn't loaded yet.
    fn get_scene_lib(&mut self, scene: SceneId) -> &Library {
        unsafe {
            //If the scene isn't loaded yet, load it and store it in the hashmap.
            if !self.loaded_libraries.contains_key(&scene) {
                self.loaded_libraries.insert(scene, load_scene(scene));
            }

            //Then get the scene from the hashmap
            let result = self.loaded_libraries.get(&scene);
            result.unwrap()
        }
    } 

    /// Get the scene data of a certain scene
    /// Scene data is not cached in the scene loader.
    /// If the library scene data exists in is not loaded, it will load and instantly unload that library.
    /// This function is unsafe, be careful when calling it.
    pub fn get_scene_data(&mut self, scene: SceneId) -> SceneData {
        let result: SceneData;

        //Check if the library was already loaded or if this function will load it.
        //This function should unload any library it loads.
        let unload_after: bool = self.loaded_libraries.contains_key(&scene);

        let lib = self.get_scene_lib(scene);

        unsafe {
            let func: Symbol<fn() -> SceneData> = lib.get(b"get_scene_data").expect(&format!("Failed to get scene data from scene library {}.", scene.get_lib()));
            result = func();
        }

        if unload_after {
            self.unload_scene(scene);
        }

        result
    }

    /// Get the scene specified
    /// Scenes are cached in addition to the library, so calling this on an already loaded scene is cheap.    
    /// This function is unsafe, be careful when calling it.
    pub fn get_scene(&mut self, scene: SceneId) -> &Box<dyn Scene> {
        if !self.loaded_scenes.contains_key(&scene) {
            let lib = self.get_scene_lib(scene);
            let result: Box<dyn Scene>;
            unsafe {
                let func: Symbol<fn() -> Box<dyn Scene>> = lib.get(b"get_scene").expect(&format!("Failed to get the scene from library {}.", scene.get_lib()));
                result = func();
            }
            self.loaded_scenes.insert(scene, result);
        }

        self.loaded_scenes.get(&scene).unwrap()
    }

    /// Unload a scene from memory.
    /// This doesn't do anything if the scene isn't loaded.
    /// If it fails to unload a scene it will crash.
    pub fn unload_scene(&mut self, scene: SceneId) {
        //Removed the cached scene and call it's unload implementation
        let loaded_scene = self.loaded_scenes.remove(&scene);
        if loaded_scene.is_some() {
            loaded_scene.unwrap().unload(get_mut_event_system());
        }

        //Remove the cached library and unload it from memory.
        let lib = self.loaded_libraries.remove(&scene);
        if lib.is_some() {
            lib.unwrap().close().expect(&format!("Failed to unload scene {}.", scene.to_string()));
        } //else: library wasn't loaded. The app shouldn't crash if it wasn't loaded to begin with, it's just odd.
    }
}

/// Load the library of the scene.
/// This function is unsafe, it does not automatically handle memory, it just loads the library.
/// Unloading and caching need to be handled seperately.
/// If the library is already loaded, it will load a second one, so don't do that.
unsafe fn load_scene(scene_to_load: SceneId) -> Library {
    Library::new(library_filename(scene_to_load.get_lib())).expect(&format!("Failed to load scene {}.", scene_to_load.to_string()))
}