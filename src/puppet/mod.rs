use crate::prelude::{scene_tree::GodotSceneTree, GodotRef};
use bevy::prelude::Component;
use godot::{engine::node::InternalMode, prelude::Node3D};

pub mod traits {
    use godot::prelude::Transform3D;

    use super::GodotSceneTree;

    pub trait Puppet: GodotSceneTree {
        /// Instantiate a new `GodotRef` of the desired godot node.
        fn instantiate() -> Self;
        fn despawn(&self);
    }

    pub trait Puppet3D: Puppet {
        fn update_transform(&self, local: Transform3D);
    }

    pub trait GodotCamera3D: Puppet3D {
        fn set_perspective(&self, fov: f64, z_near: f64, z_far: f64);
    }
}

pub mod puppet_3d {
    use godot::{
        engine::{Mesh, MeshInstance3D},
        prelude::{godot_print, Camera3D, Gd},
    };

    use crate::prelude::primitives::PrimitiveGodotMesh;

    use super::{
        traits::{Puppet, GodotCamera3D}, traits::Puppet3D, Component, GodotRef, GodotSceneTree, InternalMode, Node3D,
    };

    #[derive(Component)]
    pub struct PuppetNode3D(GodotRef);
    impl GodotSceneTree for PuppetNode3D {
        fn add_to_scene(&self) {
            self.get_active_scene()
                .expect("Couldn't get active scene")
                .add_child(self.0.get(), false, InternalMode::INTERNAL_MODE_DISABLED);
        }
    }
    impl Puppet for PuppetNode3D {
        fn instantiate() -> Self {
            Self(GodotRef::new(Node3D::new_alloc()).expect("Couldn't create new `GodotRef`."))
        }

        fn despawn(&self) {
            self.0.free();
        }
    }
    impl Puppet3D for PuppetNode3D {
        fn update_transform(&self, local: godot::prelude::Transform3D) {
            self.0.get::<Node3D>().set_transform(local);
        }
    }

    #[derive(Component)]
    pub struct PuppetCamera3D(GodotRef);
    impl GodotSceneTree for PuppetCamera3D {
        fn add_to_scene(&self) {
            self.get_active_scene()
                .expect("Couldn't get active scene")
                .add_child(self.0.get(), false, InternalMode::INTERNAL_MODE_DISABLED);
        }
    }
    impl Puppet for PuppetCamera3D {
        fn instantiate() -> Self {
            Self(GodotRef::new(Camera3D::new_alloc()).expect("Couldn't create new `GodotRef`."))
        }

        fn despawn(&self) {
            self.0.free();
        }
    }
    impl Puppet3D for PuppetCamera3D {
        fn update_transform(&self, local: godot::prelude::Transform3D) {
            self.0.get::<Node3D>().set_transform(local);
        }
    }
    impl GodotCamera3D for PuppetCamera3D {
        fn set_perspective(&self, fov: f64, z_near: f64, z_far: f64) {
            self.0.get::<Camera3D>().set_perspective(fov, z_near, z_far);
        }
    }

    #[derive(Component)]
    pub struct PuppetMeshInstance3D(GodotRef);
    impl GodotSceneTree for PuppetMeshInstance3D {
        fn add_to_scene(&self) {
            self.get_active_scene()
                .expect("Couldn't get active scene")
                .add_child(self.0.get(), false, InternalMode::INTERNAL_MODE_DISABLED);
        }
    }
    impl Puppet for PuppetMeshInstance3D {
        fn instantiate() -> Self {
            Self(
                GodotRef::new(MeshInstance3D::new_alloc())
                    .expect("Couldn't create new `GodotRef`."),
            )
        }

        fn despawn(&self) {
            self.0.free();
        }
    }
    impl Puppet3D for PuppetMeshInstance3D {
        fn update_transform(&self, local: godot::prelude::Transform3D) {
            self.0.get::<Node3D>().set_transform(local);
        }
    }
    impl PuppetMeshInstance3D {
        pub fn set_mesh(&self, mesh: Gd<Mesh>) {
            self.0.get::<MeshInstance3D>().set_mesh(mesh);
            godot_print!("Set PuppetMeshInstance3D mesh!");
        }
        pub fn from_primitive_mesh(primitive: PrimitiveGodotMesh) -> Self {
            let instance = Self::instantiate();
            instance.set_mesh(primitive.get_mesh());
            instance
        }
    }
}

pub mod puppet_3d_bundles {
    use bevy::prelude::{Bundle, Handle, Transform};

    use crate::prelude::primitives::PrimitiveGodotMesh;

    use super::puppet_3d::{PuppetCamera3D, PuppetMeshInstance3D, PuppetNode3D};

    #[derive(Bundle)]
    pub struct PuppetNode3DBundle {
        pub puppet: PuppetNode3D,
        pub transform: Transform,
    }

    #[derive(Bundle)]
    pub struct PuppetCamera3DBundle {
        pub puppet: PuppetCamera3D,
        pub transform: Transform,
    }

    #[derive(Bundle)]
    pub struct PuppetMeshInstance3DBundle {
        pub primitive: Handle<PrimitiveGodotMesh>,
        pub puppet: PuppetMeshInstance3D,
        pub transform: Transform,
    }
}
