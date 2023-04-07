use bevy::prelude::{AddAsset, Plugin};

use super::primitives::PrimitiveGodotMesh;

pub struct GodotAssetInsertPlugin;

impl Plugin for GodotAssetInsertPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<PrimitiveGodotMesh>();
    }
}
