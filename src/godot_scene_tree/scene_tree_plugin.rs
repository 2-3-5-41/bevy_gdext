use bevy::prelude::{Added, App, Changed, Plugin, Query, Transform};

use crate::prelude::{node3d::Node3DPuppet, scene_tree::GodotSceneTree, GodotTransformTranslation};

pub struct GodotSceneTreePlugin;

impl Plugin for GodotSceneTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_puppets_to_scene_tree)
            .add_system(update_puppet_transforms);
    }
}

fn add_puppets_to_scene_tree(changes: Query<&Node3DPuppet, Added<Node3DPuppet>>) {
    for puppet in &changes {
        puppet.add_to_scene();
    }
}

fn update_puppet_transforms(changes: Query<(&Transform, &Node3DPuppet), Changed<Transform>>) {
    for (transform, puppet) in &changes {
        puppet.update_transform(transform.to_godot_transform());
    }
}
