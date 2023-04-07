use bevy::{
    app::{PluginGroupBuilder, ScheduleRunnerPlugin},
    prelude::{AssetPlugin, FrameCountPlugin, PluginGroup, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};

use crate::prelude::scene_tree_plugin::GodotSceneTreePlugin;

pub struct GodotDefaultPlugins;

impl PluginGroup for GodotDefaultPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
            .add(TimePlugin::default())
            .add(ScheduleRunnerPlugin::default())
            .add(AssetPlugin::default())
            .add(GodotSceneTreePlugin)
    }
}
