use bevy::reflect::TypeUuid;
use godot::{
    engine::{BoxMesh, CapsuleMesh, CylinderMesh, Mesh, PlaneMesh, SphereMesh, TorusMesh},
    prelude::Gd,
};

use crate::prelude::GodotRef;

pub enum PrimitiveMeshType {
    Box,
    Capsule,
    Cylinder,
    Plane,
    Sphere,
    Torus,
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "74e8d7e0-7032-444e-b29a-2c8cd525a3fd"]
pub struct PrimitiveGodotMesh(GodotRef);
impl PrimitiveGodotMesh {
    pub fn new(primitive_type: PrimitiveMeshType) -> Self {
        match primitive_type {
            PrimitiveMeshType::Box => Self(
                GodotRef::new(BoxMesh::new()).expect("Could not create `GodotRef` for `BoxMesh`"),
            ),
            PrimitiveMeshType::Capsule => Self(
                GodotRef::new(CapsuleMesh::new())
                    .expect("Could not create `GodotRef` for `CapsuleMesh`"),
            ),
            PrimitiveMeshType::Cylinder => Self(
                GodotRef::new(CylinderMesh::new())
                    .expect("Could not create `GodotRef` for `CylinderMesh`"),
            ),
            PrimitiveMeshType::Plane => Self(
                GodotRef::new(PlaneMesh::new())
                    .expect("Could not create `GodotRef` for `PlaneMesh`"),
            ),
            PrimitiveMeshType::Sphere => Self(
                GodotRef::new(SphereMesh::new())
                    .expect("Could not create `GodotRef` for `SphereMesh`"),
            ),
            PrimitiveMeshType::Torus => Self(
                GodotRef::new(TorusMesh::new())
                    .expect("Could not create `GodotRef` for `TorusMesh`"),
            ),
        }
    }
    pub fn get_mesh(&self) -> Gd<Mesh> {
        self.0.get::<Mesh>()
    }
}
