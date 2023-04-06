use bevy::{prelude::{App, Commands, Transform, Query, Entity, With, Res}, MinimalPlugins, time::Time};
use bevy_gdext::prelude::{scene_tree_plugin::GodotSceneTreePlugin, puppet_3d::PuppetNode3D, traits::Puppet};
use godot::{
    engine::Engine,
    prelude::{gdextension, godot_api, Base, ExtensionLibrary, GodotClass, Node, NodeVirtual},
};

struct MyBevyApp;

#[gdextension]
unsafe impl ExtensionLibrary for MyBevyApp {}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct World {
    app: App,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for World {
    fn init(base: Base<Node>) -> Self {
        Self {
            app: App::new(),
            base,
        }
    }
    fn ready(&mut self) {
        self.app
            .add_plugins(MinimalPlugins)
            .add_plugin(GodotSceneTreePlugin)
            .add_startup_system(setup)
            .add_system(despawn_puppets)
            .setup()
    }
    fn process(&mut self, _delta: f64) {
        // Bevy/Gdext will panic when ran in editor.
        if Engine::singleton().is_editor_hint() {
            return;
        }
        self.app.update();
    }
}

// -----------------------------
// Just like normal bevy code!

fn setup(mut commands: Commands) {
    commands.spawn((
        PuppetNode3D::instantiate(),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn despawn_puppets(mut commands: Commands, search: Query<(Entity, &PuppetNode3D), With<PuppetNode3D>>, time: Res<Time>) {
    if time.elapsed_seconds() < 10.0 {
        return 
    }

    for (entity, puppet) in &search {
        puppet.despawn(); // Free from godot first. (Mem leak will happen if you skip this step!)
        commands.entity(entity).remove::<PuppetNode3D>(); // Pull bevy's hand out of the puppet's ass.
    }
}