use godot::{
    engine::Engine,
    prelude::{godot_error, godot_warn, Gd, Node, SceneTree},
};

/// utility trait to give access to Godot's scene tree.
pub trait GodotSceneTree {
    fn get_scene_tree(&self) -> Option<Gd<SceneTree>> {
        match Engine::singleton().get_main_loop() {
            Some(tree) => Some(tree.cast::<SceneTree>()),
            None => {
                godot_error!("Couldn't access scene tree!");
                None
            }
        }
    }
    fn get_active_scene(&self) -> Option<Gd<Node>> {
        if let Some(tree) = self.get_scene_tree() {
            tree.get_current_scene()
        } else {
            godot_warn!("Couldn't access current scene...");
            None
        }
    }
    fn add_to_scene(&self);
}
