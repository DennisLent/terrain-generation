use bevy::{color::{ColorToComponents, Srgba}, ecs::system::Resource};

pub const OCEAN: f32 = 30.0;
pub const DEEP_OCEAN: f32 = 10.0;
pub const SEA_LEVEL: f32 = 60.0;
pub const MAX_HEIGHT: f32 = 280.0;

#[derive(Debug, Clone, Copy)]
pub enum Biome {
    // Land Biomes
    Grassland,
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
            Biome::Grassland => Srgba::rgba_u8(91, 235, 52, 255).to_f32_array(),

            // Aquatic Biomes
            Biome::DeepOcean => Srgba::rgba_u8(12, 63, 173, 255).to_f32_array(),
            Biome::CoralReef => Srgba::rgba_u8(12, 173, 146, 255).to_f32_array(),
            Biome::ShallowSea => Srgba::rgba_u8(30, 231, 235, 255).to_f32_array(),
            Biome::ShallowOcean => Srgba::rgba_u8(48, 109, 240, 255).to_f32_array(),
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
}

#[derive(Debug, Clone, Resource)]
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