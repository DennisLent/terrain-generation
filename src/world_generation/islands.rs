use noise::{Fbm, NoiseFn, Perlin};
use super::tile::{SEA_LEVEL, OCEAN, DEEP_OCEAN, WorldMap};
/// This file generates the islands and island layer for the world
/// Bottom of the ocean = 0.0
/// Sea level = 60
/// Highest peak = 280


fn generate_islands_4096(size: u32, seed: u32) -> WorldMap {
    let mut world_map: WorldMap = WorldMap::new(size);
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
                        world_map.set_height(x as usize, z as usize, SEA_LEVEL);
                    }
                }
            } else {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        world_map.set_height(x as usize, z as usize, OCEAN);
                    }
                }
            }
        }
    }
    world_map
}

fn generate_islands(world_map: &mut WorldMap, seed: u32, chunk_size: u32) {
    let noise = Fbm::<Perlin>::new(seed);
    let size = world_map.get_size();

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
                        world_map.set_height(x as usize, z as usize, SEA_LEVEL);
                    }
                }
            } else {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        world_map.set_height(x as usize, z as usize, OCEAN);
                    }
                }
            }
        }
    }
}

fn add_deep_ocean(world_map: &mut WorldMap) {
    let size = world_map.get_size();
    let chunk_size: u32 = 256;

    // Directions to check for neighboring chunks
    let neighbor_offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for chunk_x in (0..size).step_by(chunk_size as usize) {
        for chunk_z in (0..size).step_by(chunk_size as usize) {
            let mut is_surrounded_by_ocean = true;

            // Check all neighboring chunks
            for &(dx, dz) in &neighbor_offsets {
                let neighbor_x = chunk_x as isize + dx * chunk_size as isize;
                let neighbor_z = chunk_z as isize + dz * chunk_size as isize;

                // Ensure neighbors are within bounds
                if neighbor_x >= 0
                    && neighbor_x < size as isize
                    && neighbor_z >= 0
                    && neighbor_z < size as isize
                {
                    // Check if this neighbor is ocean or below
                    let mut neighbor_is_ocean = true;
                    for x in neighbor_x as u32..(neighbor_x as u32 + chunk_size).min(size) {
                        for z in neighbor_z as u32..(neighbor_z as u32 + chunk_size).min(size) {
                            if world_map.get_height(x as usize, z as usize) > OCEAN {
                                neighbor_is_ocean = false;
                                break;
                            }
                        }
                        if !neighbor_is_ocean {
                            break;
                        }
                    }

                    // If a neighbor is not ocean, this chunk is not deep ocean
                    if !neighbor_is_ocean {
                        is_surrounded_by_ocean = false;
                        break;
                    }
                }
            }

            // If the chunk is completely surrounded by ocean, classify it as deep ocean
            if is_surrounded_by_ocean {
                for x in chunk_x..(chunk_x + chunk_size).min(size) {
                    for z in chunk_z..(chunk_z + chunk_size).min(size) {
                        world_map.set_height(x as usize, z as usize, DEEP_OCEAN);
                    }
                }
            }
        }
    }
}
pub fn island_stack(size: u32, seed: u32) -> WorldMap {
    println!("Generating islands...");
    let mut world_map = generate_islands_4096(size, seed);
    generate_islands(&mut world_map, seed, 2048);
    generate_islands(&mut world_map, seed, 256);
    println!("Generating oceans...");
    add_deep_ocean(&mut world_map);
    world_map
}