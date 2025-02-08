use bevy::prelude::*;
use bevy::render::mesh::{Mesh, VertexAttributeValues};

use super::biomes::assign_biome;
use super::islands::island_stack;
use super::climate::climate_stack;
use super::tile::{WorldMap, Biome};

fn generate_world_map(size: f32, seed: u32) -> WorldMap {
    let mut world_map = island_stack(size as u32, seed);
    climate_stack(&mut world_map, seed);
    assign_biome(&mut world_map);

    world_map
}

pub fn generate_terrain_mesh(size: f32, subdivisions: u32, seed: u32) -> (Mesh, WorldMap) {
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(size, size).subdivisions(subdivisions));

    // Generate the world map (heights, temperatures, etc.)
    let mut world_map = generate_world_map(size, seed);
    let world_size = world_map.get_size();
    let mut colors = Vec::new();

    println!("Meshing and coloring...");
    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            // Normalize the vertex positions to map to the world map size
            let normalized_x = (pos[0] / size + 0.5) * (world_size as f32 - 1.0);
            let normalized_z = (pos[2] / size + 0.5) * (world_size as f32 - 1.0);

            // Convert normalized positions to world map indices
            let x_index = normalized_x.round() as usize;
            let z_index = normalized_z.round() as usize;

            // Clamp indices to avoid out-of-bounds errors
            let x_index = x_index.clamp(0, world_size as usize - 1);
            let z_index = z_index.clamp(0, world_size as usize - 1);

            // Retrieve height from the world map
            let height = world_map.get_height(x_index, z_index);

            // Assign the height to the y-coordinate of the vertex
            pos[1] = height;

            // Assign biome-based color
            let biome_color = world_map.get_biome(x_index, z_index).unwrap_or(Biome::Grassland).color();
            colors.push(biome_color);

        }

        // Assign vertex colors to the mesh
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    println!("Recomputing normals");
    mesh.compute_normals();

    (mesh, world_map)
}