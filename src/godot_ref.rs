use godot::prelude::{godot_error, Gd, GodotClass, InstanceId, Node};

#[derive(Clone, Debug)]
pub struct GodotRef(InstanceId);
impl GodotRef {
    pub fn get<T: GodotClass>(&self) -> Gd<T> {
        return Gd::from_instance_id(self.0);
    }
    pub fn new<T: GodotClass>(reference: Gd<T>) -> Option<Self> {
        if let Some(instance) = reference.instance_id_or_none() {
            return Some(Self(instance));
        } else {
            godot_error!("Could not grab `GodotRef`!");
            return None;
        };
    }
    pub fn free(&self) {
        self.get::<Node>().free();
    }
}
