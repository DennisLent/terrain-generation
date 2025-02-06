use noise::{Fbm, NoiseFn, Perlin};


/// This file generates the islands and island layer for the world
/// Bottom of the ocean = 0.0
/// Sea level = 60
/// Highest peak = 280

fn generate_islands_4096(size: u32, seed: u32) -> Vec<Vec<f32>> {
    println!("Generating base of world with {}x{}", size, size);

    let mut world_map: Vec<Vec<f32>> = vec![vec![0.0; size as usize]; size as usize];
    let noise = Fbm::<Perlin>::new(seed);

    let chunk_size = 4096;
    for chunk_x in (0..size).step_by(chunk_size as usize) {
        for chunk_z in (0..size).step_by(chunk_size as usize) {
            // Determine the chunk center for noise evaluation
            let center_x = chunk_x as f64 + (chunk_size as f64 / 2.0);
            let center_z = chunk_z as f64 + (chunk_size as f64 / 2.0);

            // Normalize the center coordinates for noise input
            let nx = center_x / size as f64;
            let nz = center_z / size as f64;

            // Get noise value to determine if this chunk is land or water
            let noise_value = noise.get([nx, nz]);

            // If noise value > 0.0, designate this chunk as land
            if noise_value > 0.0 {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        world_map[x as usize][z as usize] = 60.0; // Set land height
                    }
                }
            } else {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        world_map[x as usize][z as usize] = 10.0; // Set land height
                    }
                }
            }
        }
    }
    world_map
}

fn generate_islands_2048(world_map: &mut Vec<Vec<f32>>, seed: u32) {
    println!("[Zoomed to 2048] Generating islands...");

    let noise = Fbm::<Perlin>::new(seed);
    let size = world_map.len(); // Assuming square world map

    let chunk_size = 2048;

    for chunk_x in (0..size).step_by(chunk_size as usize) {
        for chunk_z in (0..size).step_by(chunk_size as usize) {
            // Determine the chunk center for noise evaluation
            let center_x = chunk_x as f64 + (chunk_size as f64 / 2.0);
            let center_z = chunk_z as f64 + (chunk_size as f64 / 2.0);

            // Normalize the center coordinates for noise input
            let nx = center_x / size as f64;
            let nz = center_z / size as f64;

            // Get noise value to determine if this chunk is land or water
            let noise_value = noise.get([nx, nz]);

            // Determine if this chunk is land or water
            if noise_value > 0.0 {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        // Only update if it's not already land
                        if world_map[x as usize][z as usize] < 60.0 {
                            world_map[x as usize][z as usize] = 60.0; // Set land height
                        }
                    }
                }
            } else {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        // Only update if it's not already land
                        if world_map[x as usize][z as usize] < 60.0 {
                            world_map[x as usize][z as usize] = 10.0; // Set water height
                        }
                    }
                }
            }
        }
    }
}

pub fn island_stack(size: u32, seed: u32) -> Vec<Vec<f32>> {
    let mut world_map = generate_islands_4096(size, seed);
    generate_islands_2048(&mut world_map, seed);
    world_map
}