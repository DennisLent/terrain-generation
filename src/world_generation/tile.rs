pub const OCEAN: f32 = 30.0;
pub const DEEP_OCEAN: f32 = 10.0;
pub const SEA_LEVEL: f32 = 60.0;
pub const MAX_HEIGHT: f32 = 280.0;

#[derive(Debug, Clone, Copy)]
pub enum Biome {
    // Land Biomes
    TropicalRainforest,
    TropicalForest,
    Savanna,
    Desert,
    SubtropicalDesert,
    TemperateRainforest,
    TemperateForest,
    Grassland,
    Woodland,
    Taiga,
    Tundra,
    TemperateDesert,
    ArcticDesert,
    // Aquatic Biomes
    DeepOcean,
    CoralReef,
    ShallowSea,
    ShallowOcean,
}

impl Biome {
    pub fn color(&self) -> [f32; 4] {
        match self {
            // Land Biomes
            Biome::TropicalRainforest => [0.0 / 255.0, 128.0 / 255.0, 0.0, 1.0], // Dark Green
            Biome::TropicalForest => [34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0], // Forest Green
            Biome::Savanna => [238.0 / 255.0, 232.0 / 255.0, 170.0 / 255.0, 1.0], // Light Yellow-Green
            Biome::Desert => [210.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0, 1.0], // Sandy Brown
            Biome::SubtropicalDesert => [244.0 / 255.0, 164.0 / 255.0, 96.0 / 255.0, 1.0], // Light Orange
            Biome::TemperateRainforest => [0.0 / 255.0, 100.0 / 255.0, 0.0, 1.0], // Teal Green
            Biome::TemperateForest => [34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0], // Medium Green
            Biome::Grassland => [189.0 / 255.0, 183.0 / 255.0, 107.0 / 255.0, 1.0], // Beige
            Biome::Woodland => [154.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0, 1.0], // Light Olive
            Biome::Taiga => [0.0 / 255.0, 128.0 / 255.0, 255.0 / 255.0, 1.0], // Dark Cyan
            Biome::Tundra => [175.0 / 255.0, 207.0 / 255.0, 223.0 / 255.0, 1.0], // Light Cyan
            Biome::TemperateDesert => [244.0 / 255.0, 164.0 / 255.0, 96.0 / 255.0, 1.0], // Light Orange
            Biome::ArcticDesert => [220.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0, 1.0], // Pale Cyan

            // Aquatic Biomes
            Biome::DeepOcean => [4.0 / 255.0, 11.0 / 255.0, 89.0 / 255.0, 1.0], // Dark Blue
            Biome::CoralReef => [255.0 / 255.0, 160.0 / 255.0, 122.0 / 255.0, 1.0], // Light Coral
            Biome::ShallowSea => [64.0 / 255.0, 164.0 / 255.0, 223.0 / 255.0, 1.0], // Light Blue
            Biome::ShallowOcean => [0.0 / 255.0, 105.0 / 255.0, 148.0 / 255.0, 1.0], // Medium Blue
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub height: f32,
    pub temperature: f32,
    pub rainfall: f32,
    pub biome: Option<Biome>
}

impl Tile {
    pub fn new() -> Self {
        Tile { height: 0.0, temperature: 0.0, rainfall: 0.0, biome: None}
    }

    pub fn set_temperature(&mut self, temp: f32) {
        self.temperature = temp;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn set_rainfall(&mut self, rainfall: f32) {
        self.rainfall = rainfall;
    }

    pub fn set_biome(&mut self, biome: Biome) {
        self.biome = Some(biome)
    }
}

#[derive(Debug, Clone)]
pub struct WorldMap {
    size: u32,
    pub tiles: Vec<Vec<Tile>>
}

impl WorldMap {
    pub fn new(size: u32) -> Self {
        WorldMap { size: size, tiles: vec![vec![Tile::new(); size as usize]; size as usize] }
    }

    pub fn get_height(&mut self, x: usize, z: usize) -> f32 {
        self.tiles[x][z].height
    }

    pub fn set_height(&mut self, x: usize, z: usize, height: f32) {
        self.tiles[x][z].height = height
    }

    pub fn get_temperature(&mut self, x: usize, z: usize) -> f32 {
        self.tiles[x][z].temperature
    }

    pub fn set_temperature(&mut self, x: usize, z: usize, temperature: f32) {
        self.tiles[x][z].temperature = temperature
    }

    pub fn get_rainfall(&mut self, x: usize, z: usize) -> f32 {
        self.tiles[x][z].rainfall
    }

    pub fn set_rainfall(&mut self, x: usize, z: usize, rainfall: f32) {
        self.tiles[x][z].rainfall = rainfall
    }

    pub fn get_size(&mut self) -> u32 {
        self.size
    }

    pub fn get_biome(&mut self, x: usize, z: usize) -> Option<Biome> {
        self.tiles[x][z].biome
    }

    pub fn set_biome(&mut self, x: usize, z: usize, biome: Biome) {
        self.tiles[x][z].biome = Some(biome)
    }
}