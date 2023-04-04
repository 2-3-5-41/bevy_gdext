use bevy::{
    prelude::{Bundle, Component},
    transform::TransformBundle,
};
use godot::{
    engine::{self, node::InternalMode},
    prelude::{godot_print, Node3D, Transform3D},
};

use crate::prelude::{scene_tree::GodotSceneTree, GodotRef};

pub trait Node3DPuppetTrait: GodotSceneTree {
    fn sync_transform(&self);
}

/// Bevy component that is used to track and update the instance of the created
/// `Node3D` instance from Bevy.
#[derive(Component, Debug)]
pub struct Node3DPuppet(GodotRef);

impl Node3DPuppet {
    pub fn new() -> Self {
        Self(GodotRef::new(engine::Node3D::new_alloc()).expect("Could not create `GodotRef`"))
    }
    pub fn update_transform(&self, local: Transform3D) {
        self.0.get::<Node3D>().set_transform(local);
    }
}

impl GodotSceneTree for Node3DPuppet {
    fn add_to_scene(&self) {
        self.get_active_scene().unwrap().add_child(
            self.0.get(),
            true,
            InternalMode::INTERNAL_MODE_DISABLED,
        );

        godot_print!("Added: {:?} \n to the scene tree from Bevy.", self);
    }
}

/// Bevy bundle to create a `Node3D` instance for Godot.
#[derive(Bundle)]
pub struct Node3DBundle {
    pub instance: Node3DPuppet,
    pub transform: TransformBundle,
}

impl Default for Node3DBundle {
    fn default() -> Self {
        Self {
            instance: Node3DPuppet::new(),
            transform: TransformBundle::default(),
        }
    }
}
