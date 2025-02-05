use bevy::prelude::*;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use noise::{BasicMulti, NoiseFn, Perlin};
use rand::Rng;

/// Generates a temperature map for the world
fn generate_temperature_map(size: usize, noise_scale: f64, seed: u32) -> Vec<Vec<f64>> {
    let noise = BasicMulti::<Perlin>::new(seed);
    let mut map = vec![vec![0.0; size]; size];

    for x in 0..size {
        for z in 0..size {
            let val = noise.get([x as f64 * noise_scale, z as f64 * noise_scale]);
            map[x][z] = val.clamp(-1.0, 1.0); // Normalize to [-1, 1] range
        }
    }

    map
}

/// Generates a rainfall map for the world
fn generate_rainfall_map(size: usize, noise_scale: f64, seed: u32) -> Vec<Vec<f64>> {
    let noise = BasicMulti::<Perlin>::new(seed);
    let mut map = vec![vec![0.0; size]; size];

    for x in 0..size {
        for z in 0..size {
            let val = noise.get([x as f64 * noise_scale, z as f64 * noise_scale]);
            map[x][z] = val.clamp(0.0, 1.0); // Normalize to [0, 1] range
        }
    }

    map
}

/// Assigns a color based on the height, temperature and rainfall
fn assign_biome(height: f64, rainfall: f64, temperature: f64, height_multiplier: f32) -> [f32; 4] {
    // Normalize height based on height_multiplier
    let height = height / height_multiplier as f64;

    if height > 0.6 {
        // White for high peaks
        [1.0, 1.0, 1.0, 1.0]
    } else if height > 0.3 {
        // Brown for mountainy areas
        [139.0 / 255.0, 69.0 / 255.0, 19.0 / 255.0, 1.0]
    } else {
        // Green for everything else
        [34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0]
    }
}


/// Generates a terrain mesh with Perlin noise applied.
pub fn generate_terrain_mesh(size: f32, subdivisions: u32, height_multiplier: f32, noise_scale: f64, seed: Option<u32>) -> Mesh {
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(size, size).subdivisions(subdivisions));

    let mut heights = Vec::new();
    let mut colors = Vec::new();

    // Assign seeds: If provided, use the seed, otherwise default to (1, 2, 3)
    let base_seed = seed.unwrap_or(1);
    let height_seed = base_seed;
    let rainfall_seed = base_seed + 1;
    let temperature_seed = base_seed + 2;

    // Create multi-layered Perlin noise generators
    let base_noise = Perlin::new(height_seed);
    let detail_noise = Perlin::new(height_seed + 1);

    // Create Perlin noise generators with specified seeds
    let rainfall_noise = BasicMulti::<Perlin>::new(rainfall_seed);
    let temperature_noise = BasicMulti::<Perlin>::new(temperature_seed);

    // Apply Perlin noise to the vertex positions
    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {


        for pos in positions.iter_mut() {
            let x = pos[0] as f64 * noise_scale;
            let z = pos[2] as f64 * noise_scale;

            // Large-scale base terrain (low frequency, smooth rolling hills)
            let mut height_val = base_noise.get([x * 0.5, z * 0.5]) * 0.4;

            // Medium-scale terrain variations (hills)
            height_val += base_noise.get([x * 1.5, z * 1.5]) * 0.6;

            // Small-scale details (rocky areas and finer terrain variation)
            height_val += detail_noise.get([x * 6.0, z * 6.0]) * 0.2;

            // Normalize the height and apply a power function to create more plains
            height_val = height_val.powf(1.1); // Adjust exponent to flatten terrain

            // Apply the final height value
            if height_val.is_nan(){
                let mut rng = rand::rng();
                pos[1] = rng.random_range(0.0..0.03) * height_multiplier;
            }
            else{
                pos[1] = height_val as f32 * height_multiplier;
            }

            // Generate rainfall and temperature values
            let rainfall = rainfall_noise.get([pos[0] as f64 * noise_scale, pos[2] as f64 * noise_scale]);
            let temperature = temperature_noise.get([pos[0] as f64 * noise_scale, pos[2] as f64 * noise_scale]);

            // Assign biome-based color
            let biome_color = assign_biome(
                pos[1] as f64,
                rainfall.clamp(0.0, 1.0),
                temperature.clamp(-1.0, 1.0),
                height_multiplier,
            );
            colors.push(biome_color);
            heights.push(pos[1]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    // Recompute normals for proper shading
    mesh.compute_normals();

    mesh
}