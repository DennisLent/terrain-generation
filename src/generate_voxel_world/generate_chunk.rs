use super::noise;
use super::terrain;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use noise::generate_heightmap;
use terrain::{TerrainVoxel, CHUNK_SIZE, EMPTY, FULL};

pub const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// Generate the chunk, based on the chunk index and the seed.
/// Uses `ConstShape3u32` for delinearizing indices to simplify traversal.
pub fn generate_chunk(
    chunk_x_index: i32,
    chunk_z_index: i32,
    seed: u32,
) -> [TerrainVoxel; CHUNK_VOLUME] {
    // Define the chunk shape.
    type ChunkShape = ConstShape3u32<CHUNK_SIZE_U32, CHUNK_SIZE_U32, CHUNK_SIZE_U32>;

    // Generate the heightmap for the chunk.
    let heightmap = generate_heightmap(chunk_x_index, chunk_z_index, seed);

    // Initialize the voxel array for the chunk.
    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];

    // Iterate through all indices in the chunk using the shape's delinearize method.
    for i in 0..ChunkShape::SIZE {
        // Get the (x, y, z) coordinates for the current index.
        let [x, y, z] = ChunkShape::delinearize(i);

        // Map the chunk coordinates (x, z) to world height values using the heightmap.
        let height = heightmap[x as usize][z as usize] as u32;

        // Determine if this voxel is solid or air based on its height.
        voxels[i as usize] = if y <= height {
            FULL // Solid voxel
        } else {
            EMPTY // Air voxel
        };

        if x == 0 || x == CHUNK_SIZE_U32 - 1 || z == 0 || z == CHUNK_SIZE_U32 - 1 {
            voxels[i as usize] = FULL;
        }
    }

    // println!("Chunk at {}, {} generated with {} solid voxels", chunk_x_index, chunk_z_index, solid_voxel_count);

    voxels
}
