use bevy::prelude::*;
use bevy::render::mesh::{Mesh, VertexAttributeValues};

use super::biome_colors::assign_biome;
use super::islands::island_stack;



pub fn generate_terrain_mesh(size: f32, subdivisions: u32, height_multiplier: f32, seed: u32) -> Mesh {
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(size, size).subdivisions(subdivisions));

    let heights = island_stack(size as u32, seed);
    let mut colors = Vec::new();

    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {

            // Normalize the vertex positions to map to the heightmap size
            let normalized_x = (pos[0] / size + 0.5) * (heights.len() as f32 - 1.0);
            let normalized_z = (pos[2] / size + 0.5) * (heights.len() as f32 - 1.0);

            // Convert normalized positions to heightmap indices
            let x_index = normalized_x.round() as usize;
            let z_index = normalized_z.round() as usize;

            // Clamp indices to avoid out-of-bounds errors
            let x_index = x_index.clamp(0, heights.len() - 1);
            let z_index = z_index.clamp(0, heights[0].len() - 1);

            // Debug: Print mapped indices and height
            if x_index == 0 && z_index == 0 {
                println!("Mapped Vertex ({}, {}) -> Indices ({}, {}) -> Height {}", pos[0], pos[2], x_index, z_index, heights[x_index][z_index]);
            }

            // Assign height from the heightmap
            pos[1] = heights[x_index][z_index];

            // Assign biome-based color
            let biome_color = assign_biome(
                pos[1] as f64,
                1.0,
                1.0,
                height_multiplier,
            );
            colors.push(biome_color);
        }

        // Assign vertex colors to the mesh
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    println!("Recomputing normals");
    mesh.compute_normals();

    mesh
}