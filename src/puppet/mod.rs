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
}

pub mod puppet_3d {

    use super::{
        traits::Puppet, traits::Puppet3D, Component, GodotRef, GodotSceneTree, InternalMode, Node3D,
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
}
