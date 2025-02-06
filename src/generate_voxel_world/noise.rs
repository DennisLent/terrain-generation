use super::terrain;
use noise::{BasicMulti, NoiseFn, Perlin};
use terrain::CHUNK_SIZE;

/// Use perlin noise to generate heightmap for the voxel world
/// Use the chunk index in the world and seed to get a heighmap for each
pub fn generate_heightmap(
    chunk_x_index: i32,
    chunk_z_index: i32,
    seed: u32,
) -> [[f32; CHUNK_SIZE]; CHUNK_SIZE] {
    let noise = BasicMulti::<Perlin>::new(seed);
    let mut heightmap = [[0.0; CHUNK_SIZE]; CHUNK_SIZE];

    // println!("Heightmap for chunk ({}, {})", chunk_x_index, chunk_z_index);

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let world_x = chunk_x_index * CHUNK_SIZE as i32 + x as i32;
            let world_z = chunk_z_index * CHUNK_SIZE as i32 + z as i32;

            // adjust the scale of the world and add 20 on top to ensure we have a solid ground
            let scale = 0.01;
            let height = (noise.get([world_x as f64 * scale, world_z as f64 * scale]) * 10.0) + 20.0;
            heightmap[x][z] = height as f32;

        }
    }

    //print heightmap or debug:

    heightmap
}
