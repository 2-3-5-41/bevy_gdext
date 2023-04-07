use bevy::{
    prelude::{App, Assets, Commands, Entity, Query, Res, ResMut, Transform, Vec3, With},
    time::Time,
};
use bevy_gdext::prelude::{
    asset_insert_plugin::GodotAssetInsertPlugin,
    primitives::{PrimitiveGodotMesh, PrimitiveMeshType},
    puppet_3d::{PuppetCamera3D, PuppetMeshInstance3D},
    puppet_3d_bundles::{PuppetCamera3DBundle, PuppetMeshInstance3DBundle},
    traits::Puppet,
    GodotDefaultPlugins,
};
use godot::{
    engine::Engine,
    prelude::{
        gdextension, godot_api, Base, ExtensionLibrary, GodotClass, Node, NodeVirtual,
    },
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
        // std::env::set_var("RUST_BACKTRACE", "1");
        Self {
            app: App::new(),
            base,
        }
    }
    fn ready(&mut self) {
        self.app
            .add_plugins(GodotDefaultPlugins)
            .add_plugin(GodotAssetInsertPlugin)
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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<PrimitiveGodotMesh>>) {
    // Spawn camera3d node
    commands.spawn(PuppetCamera3DBundle {
        puppet: PuppetCamera3D::instantiate(),
        transform: Transform::from_xyz(1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
    });

    let new_mesh_handle = meshes.add(PrimitiveGodotMesh::new(PrimitiveMeshType::Box));
    commands.spawn(PuppetMeshInstance3DBundle {
        primitive: new_mesh_handle.clone(),
        puppet: match meshes.get(&new_mesh_handle) {
            Some(mesh) => PuppetMeshInstance3D::from_primitive_mesh(mesh.clone()),
            None => PuppetMeshInstance3D::instantiate(),
        },
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
    });
}

fn despawn_puppets(
    mut commands: Commands,
    search: Query<(Entity, &PuppetMeshInstance3D), With<PuppetMeshInstance3D>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() < 10.0 {
        return;
    }

    for (entity, puppet) in &search {
        puppet.despawn(); // Free from godot first. (Mem leak will happen if you skip this step!)
        commands.entity(entity).remove::<PuppetMeshInstance3D>(); // Pull bevy's hand out of the puppet's ass.
    }
}
