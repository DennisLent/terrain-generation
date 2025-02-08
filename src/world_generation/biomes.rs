use super::tile::{OCEAN, SEA_LEVEL, Biome, WorldMap};

/// Assigns a biome to each tile in the world map based on height, temperature, and rainfall
pub fn assign_biome(world_map: &mut WorldMap) {
    println!("Assigning Biomes...");
    let size = world_map.get_size() as usize;

    for x in 0..size {
        for z in 0..size {
            let tile = &mut world_map.tiles[x][z];
            let height = tile.height;
            let temperature = tile.temperature;
            let rainfall = tile.rainfall;

            tile.biome = Some(
                if height < OCEAN {
                    Biome::DeepOcean
                } else if height < SEA_LEVEL {
                    // Assign shallow water biomes
                    if rainfall > 0.8 && temperature > 0.6 {
                        Biome::CoralReef
                    } else if rainfall > 0.6 {
                        Biome::ShallowSea
                    } else {
                        Biome::ShallowOcean
                    }
                } else {
                    Biome::Grassland
                },
            );
        }
    }
}
