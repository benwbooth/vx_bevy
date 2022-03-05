use bevy::prelude::{Color, Plugin};

use crate::voxel::material::{MaterialRegistryInfo, VoxelMaterialFlags, VoxelMaterialRegistry};

pub struct Dirt;

impl Dirt {
    pub const ID: u8 = 1;
    pub const NAME: &'static str = "Dirt";
}

pub struct Sand;
impl Sand {
    pub const ID: u8 = 2;
    pub const NAME: &'static str = "Sand";
}

pub struct Grass;
impl Grass {
    pub const ID: u8 = 3;
    pub const NAME: &'static str = "Grass";
}

pub struct Rock;
impl Rock {
    pub const ID: u8 = 4;
    pub const NAME: &'static str = "Rock";
}

pub struct Snow;
impl Snow {
    pub const ID: u8 = 5;
    pub const NAME: &'static str = "Snow";
}

pub struct Water;
impl Water {
    pub const ID: u8 = 6;
    pub const NAME: &'static str = "Water";
}

pub struct VoxelWorldBaseMaterialsPlugin;

impl Plugin for VoxelWorldBaseMaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut registry = app
            .world
            .get_resource_mut::<VoxelMaterialRegistry>()
            .unwrap();

        registry.register_material::<Dirt>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(112, 97, 92),
            name: Dirt::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Sand>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(228, 219, 148),
            name: Sand::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Grass>(MaterialRegistryInfo {
            base_color: Color::LIME_GREEN,
            name: Grass::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Rock>(MaterialRegistryInfo {
            base_color: Color::GRAY,
            name: Rock::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Snow>(MaterialRegistryInfo {
            base_color: Color::WHITE,
            name: Snow::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Water>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(106, 235, 187),
            name: Water::NAME,
            flags: VoxelMaterialFlags::LIQUID,
        });
    }
}
