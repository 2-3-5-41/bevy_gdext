use godot::prelude::{Basis, Quaternion, Vector3};

pub trait GodotTransformTranslation {
    fn to_godot_transform(self) -> godot::prelude::Transform3D;
}

impl GodotTransformTranslation for bevy::prelude::Transform {
    fn to_godot_transform(self) -> godot::prelude::Transform3D {
        let [x, y, z, w] = self.rotation.to_array();
        let quat = Quaternion::new(x, y, z, w);

        let [x, y, z] = self.scale.to_array();
        let scale = Vector3::new(x, y, z);

        let basis = Basis::from_quat(quat).scaled(scale);

        let [x, y, z] = self.translation.to_array();
        godot::prelude::Transform3D {
            basis,
            origin: Vector3::new(x, y, z),
        }
    }
}
