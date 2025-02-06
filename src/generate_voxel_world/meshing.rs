use super::generate_chunk;
use super::terrain;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use block_mesh::{
    greedy_quads, ndshape::ConstShape3u32, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};
use generate_chunk::{CHUNK_SIZE_U32, CHUNK_VOLUME};
use terrain::{TerrainVoxel, CHUNK_SIZE, FULL};

const CHUNK_SHAPE: ConstShape3u32<CHUNK_SIZE_U32, CHUNK_SIZE_U32, CHUNK_SIZE_U32> = ConstShape3u32;

pub fn mesh_chunk(chunk: &[TerrainVoxel; CHUNK_VOLUME]) -> Mesh {
    // Create a buffer for greedy quads
    let mut buffer = GreedyQuadsBuffer::new(chunk.len());

    // let solid_voxel_count = chunk.iter().filter(|v| **v == FULL).count();
    // println!("Meshing chunk with {} solid voxels", solid_voxel_count);

    // Perform greedy meshing
    greedy_quads(
        chunk,
        &CHUNK_SHAPE,
        [0; 3],
        [CHUNK_SIZE as u32; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    // println!("Greedy quads found {} groups", buffer.quads.num_quads());

    // Prepare vectors for mesh data
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (group_index, group) in buffer.quads.groups.iter().enumerate() {
        let face = &RIGHT_HANDED_Y_UP_CONFIG.faces[group_index];

        for quad in group {
            let start_index = vertices.len() as u32;

            // Add quad vertices, normals, UVs, and indices
            vertices.extend(face.quad_mesh_positions(quad, 1.0));
            normals.extend(face.quad_mesh_normals());
            uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            indices.extend(face.quad_mesh_indices(start_index));
        }
    }

    // DEBUG: Print number of vertices
    // println!("Generated mesh with {} vertices", vertices.len());


    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // Insert attributes
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // **Fix: Replace `set_indices()` with `insert_indices()`**
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
