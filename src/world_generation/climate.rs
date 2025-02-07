use noise::{NoiseFn, Perlin, Simplex};
use super::tile::WorldMap;

/// Generates the climate of a region based on temperate and rainfall
/// Temperature can be from -1.0 (freezing) to 1.0 (Warm)
/// Rainfall can be from 0.0 (arid) to 1.0 (Wet)
pub fn generate_climate_layer(world_map: &mut WorldMap, seed: u32) {
    let size = world_map.get_size();
    let temperature_noise = Simplex::new(seed);
    let rainfall_noise = Simplex::new(seed + 1);
    let chunk_size = 2048;

    // Generate temperature and rainfall at the chunk level
    for chunk_x in (0..size).step_by(chunk_size as usize) {
        for chunk_z in (0..size).step_by(chunk_size as usize) {
            let nx = chunk_x as f64 / size as f64;
            let nz = chunk_z as f64 / size as f64;

            // Can take raw temperature values from Perlin noise
            let temperature_value = temperature_noise.get([nx, nz]);
            // Need to normalize the Perlin noise for rainfall
            let rainfall_value = (rainfall_noise.get([nx, nz]) + 1.0) / 2.0;

            let assigned_temperature = if temperature_value > 0.3 {
                // Warm temperature
                1.0
            } else if temperature_value > -0.2 {
                // Cold temperature
                -0.3
            } else {
                // Freezing temperature
                -1.0
            };

            for x in chunk_x..(chunk_x + chunk_size).min(size) {
                for z in chunk_z..(chunk_z + chunk_size).min(size) {
                    world_map.set_temperature(x as usize, z as usize, assigned_temperature);
                    world_map.set_rainfall(x as usize, z as usize, rainfall_value as f32);
                }
            }
        }
    }
}

/// Blends the climate zones at different zoom levels
/// We take the average temperature and rainfall in a chunk and assign it to that region
fn blend_climate(world_map: &mut WorldMap, chunk_size: u32) {
    let size = world_map.get_size();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut blended_map = world_map.tiles.clone();

    for chunk_x in (0..size).step_by(chunk_size as usize) {
        for chunk_z in (0..size).step_by(chunk_size as usize) {
            let mut total_temp = 0.0;
            let mut total_rainfall = 0.0;
            let mut count = 0;

            // Collect neighboring values
            for (dx, dz) in directions.iter() {
                let nx = chunk_x as isize + dx * chunk_size as isize;
                let nz = chunk_z as isize + dz * chunk_size as isize;

                if nx >= 0 && nz >= 0 && nx < size as isize && nz < size as isize {
                    total_temp += world_map.get_temperature(nx as usize, nz as usize);
                    total_rainfall += world_map.get_rainfall(nx as usize, nz as usize);
                    count += 1;
                }
            }

            // Calculate the blended values
            let current_temp = world_map.get_temperature(chunk_x as usize, chunk_z as usize);
            let current_rainfall = world_map.get_rainfall(chunk_x as usize, chunk_z as usize);

            let blended_temp = (current_temp + total_temp) / (count as f32 + 1.0);
            let blended_rainfall = (current_rainfall + total_rainfall) / (count as f32 + 1.0);

            for x in chunk_x..(chunk_x + chunk_size).min(size) {
                for z in chunk_z..(chunk_z + chunk_size).min(size) {
                    blended_map[x as usize][z as usize].temperature = blended_temp;
                    blended_map[x as usize][z as usize].rainfall = blended_rainfall;
                }
            }
        }
    }

    world_map.tiles = blended_map;
}

pub fn climate_stack(world_map: &mut WorldMap, seed: u32) {
    println!("Generating climate...");
    generate_climate_layer(world_map, seed);

    println!("Blending climate...");
    blend_climate(world_map, 512);
    blend_climate(world_map, 256);
}