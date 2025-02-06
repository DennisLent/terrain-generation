use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

pub const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TerrainType {
    Solid,
    Air,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TerrainVoxel(TerrainType);

pub const EMPTY: TerrainVoxel = TerrainVoxel(TerrainType::Air);
pub const FULL: TerrainVoxel = TerrainVoxel(TerrainType::Solid);

impl Voxel for TerrainVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for TerrainVoxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}
