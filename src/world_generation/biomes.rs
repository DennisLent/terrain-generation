use super::tile::{DEEP_OCEAN, OCEAN, SEA_LEVEL, Biome, WorldMap};

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
                    // Assign terrestrial biomes based on Whittaker Biome Diagram
                    match (temperature, rainfall) {
                        // Tropical Rainforest
                        (t, r) if t > 0.8 && r > 0.8 => Biome::TropicalRainforest,
                        // Tropical Seasonal Forest
                        (t, r) if t > 0.7 && r > 0.6 && r <= 0.8 => Biome::TropicalForest,
                        // Savanna
                        (t, r) if t > 0.6 && r > 0.2 && r <= 0.6 => Biome::Savanna,
                        // Desert
                        (t, r) if t > 0.5 && r <= 0.2 => Biome::Desert,
                        // Subtropical Desert
                        (t, r) if t > 0.4 && r <= 0.3 => Biome::SubtropicalDesert,
                        // Temperate Rainforest
                        (t, r) if t > 0.2 && t <= 0.4 && r > 0.7 => Biome::TemperateRainforest,
                        // Temperate Forest
                        (t, r) if t > 0.1 && t <= 0.4 && r > 0.4 && r <= 0.7 => Biome::TemperateForest,
                        // Grassland
                        (t, r) if t > 0.1 && t <= 0.4 && r <= 0.4 => Biome::Grassland,
                        // Woodland/Shrubland
                        (t, r) if t > -0.2 && t <= 0.2 && r > 0.3 => Biome::Woodland,
                        // Taiga
                        (t, r) if t > -0.5 && t <= -0.2 && r > 0.3 => Biome::Taiga,
                        // Tundra
                        (t, r) if t <= -0.5 && r <= 0.3 => Biome::Tundra,
                        // Temperate Desert
                        (t, r) if t > -0.2 && t <= 0.5 && r <= 0.2 => Biome::TemperateDesert,
                        // Arctic Desert
                        (t, r) if t < -0.5 && r <= 0.1 => Biome::ArcticDesert,
                        _ => Biome::Grassland, // Default biome
                    }
                },
            );
        }
    }
}
