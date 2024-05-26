use crate::river::create_rivers;
use crate::gen_utils::{add_islands, generate_land_map, zoom_int, zoom_float, add_height, add_temperature, add_rainfall, smooth_map, add_oceans};
use rayon::prelude::*;

//resources
// https://www.alanzucconi.com/2022/06/05/minecraft-world-generation/
// https://www.youtube.com/watch?v=YyVAaJqYAfE&t=1550s

/// Map struct to hold all the key information about the generated world
#[derive(Debug)]
pub struct Map {
    //land_map holds information about land and oceans 
    //1 = land, 2 = shallow ocean, 0 = normal ocean, -1 = deep ocean
    pub land_map: Vec<Vec<i32>>,

    //height_map holds information about the individual height of each point
    pub height_map: Vec<Vec<f32>>,

    pub temperature_map: Vec<Vec<f32>>,
    pub rainfall_map: Vec<Vec<f32>>,
}

impl Map {
    /// debugging don't mind this
    pub fn print(&self) {
        let map_size = self.land_map.len() as usize;

        println!("Land_val, height, temp, rain");
    
        for i in 0..map_size {
            for j in 0..map_size {
                println!("{} {} {} {}", self.land_map[i][j], self.height_map[i][j], self.temperature_map[i][j], self.rainfall_map[i][j]);
            }
        }
    }

    /// main method to be called when generating a map
    /// creates the land, temperature, rainfall and height map used for rendering
    // TODO: add blending of biomes
    // TODO: add imperfections along so coastline is not too square
    pub fn new(size: usize) -> Self {

        // create land map 4096 -> 2048
        println!("GENERATING LANDMASSES");
        let mut land_2048 = generate_land_map(size);

        // zoom once more 2048 -> 1024
        // add islands
        // add temperatures
        let mut land_1024 = zoom_int(land_2048);
        add_islands(&mut land_1024);
        add_islands(&mut land_1024);
        add_islands(&mut land_1024);

        println!("GENERATING TEMPERATURES");
        let mut temp_1024: Vec<Vec<f32>> = land_1024.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_temperature(&mut temp_1024);

        // 1024 -> 512
        let land_512 = zoom_int(land_1024);
        let temp_512 = zoom_float(temp_1024);

        // 512 -> 256
        // create hills to height map
        // add islands
        // add deep oceans and shallow oceans
        println!("GENERATING TERRAIN AND BIOMES");
        let mut land_256 = zoom_int(land_512);
        let temp_256 = zoom_float(temp_512);
        let mut rain_256: Vec<Vec<f32>> = land_256.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_rainfall(&mut rain_256);
        add_oceans(&mut land_256);

        // 256 -> 128 -> 64
        // add islan64
        println!("ADDING HEIGHTS");
        let mut land_64 = zoom_int(zoom_int(land_256));
        let temp_64 = zoom_float(zoom_float(temp_256));
        let rain_64 = zoom_float(zoom_float(rain_256));
        let mut height_64: Vec<Vec<f32>> = land_64.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_height(&mut height_64);

        // 64 -> 32 -> 16
        println!("ADDING RIVERS AND EDGES");
        let mut land_8 = zoom_int(zoom_int(land_64));
        let temp_8 = smooth_map(zoom_float(zoom_float(temp_64)));
        let rain_8 = smooth_map(zoom_float(zoom_float(rain_64)));
        let height_8 = smooth_map(zoom_float(zoom_float(height_64)));
        create_rivers(&height_8, &rain_8, &mut land_8, 5);

        // 16 -> 8
        // add erosion to coastline and rivers
        // smooth everything
        println!("FINAL TOUCHES");
        let final_land = zoom_int(land_8);
        let final_temp = smooth_map(zoom_float(smooth_map(temp_8)));
        let final_rain = smooth_map(zoom_float(smooth_map(rain_8)));
        let final_height = smooth_map(zoom_float(smooth_map(height_8)));


        Map {
            land_map: final_land,
            height_map: final_height,
            temperature_map: final_temp,
            rainfall_map: final_rain,
        }
    }

    pub fn board_size(&self) -> usize{
        self.land_map.len()*self.land_map.len()
    }
}
