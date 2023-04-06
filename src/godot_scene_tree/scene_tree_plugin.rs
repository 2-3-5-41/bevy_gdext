use bevy::prelude::{Added, App, Changed, Commands, Plugin, Query, RemovedComponents, Transform};

use crate::prelude::{puppet_3d::PuppetNode3D, traits::Puppet3D, GodotTransformTranslation};

use super::scene_tree::GodotSceneTree;

pub struct GodotSceneTreePlugin;

impl Plugin for GodotSceneTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_scene_tree_instantiations)
            .add_system(sync_scene_tree_removals)
            .add_system(sync_scene_tree_transforms);
    }
}

fn sync_scene_tree_instantiations(
    puppet_node3d_instantiations: Query<&PuppetNode3D, Added<PuppetNode3D>>,
) {
    for instantiation in &puppet_node3d_instantiations {
        instantiation.add_to_scene();
    }
}

fn sync_scene_tree_removals(
    mut commands: Commands,
    mut puppet_node3d_removals: RemovedComponents<PuppetNode3D>,
) {
    for entity in puppet_node3d_removals.iter() {
        commands.entity(entity).despawn();
    }
}

fn sync_scene_tree_transforms(updates: Query<(&Transform, &PuppetNode3D), Changed<Transform>>) {
    for (transform, puppet) in &updates {
        puppet.update_transform(transform.to_godot_transform());
    }
}
