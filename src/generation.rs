use crate::river::create_rivers;
use crate::gen_utils::{add_islands, generate_land_map, zoom_int, zoom_float, add_height, add_temperature, add_rainfall, smooth_map};
use rayon::prelude::*;

//resources
// https://www.alanzucconi.com/2022/06/05/minecraft-world-generation/
// https://www.youtube.com/watch?v=YyVAaJqYAfE&t=1550s
#[derive(Debug)]
pub struct Map {
    pub land_map: Vec<Vec<i32>>,
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
    // TODO: add timing to the print statements to see which steps take the longest to improve performance
    pub fn new(size: usize) -> Self {

        //create land map 4096 -> 1024
        println!("GENERATING LANDMASSES");
        let mut land_map = generate_land_map(size);

        //zoom once more 1024 -> 512
        add_islands(&mut land_map);
        add_islands(&mut land_map);
        add_islands(&mut land_map);

        //add temperates and rainfall using noise
        println!("GENERATING BIOMES");
        let mut temp_map: Vec<Vec<f32>> = land_map.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_temperature(&mut temp_map);
        let mut rain_map: Vec<Vec<f32>> = land_map.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_rainfall(&mut rain_map);

        // zoom maps again
        // 512 -> 256
        let zoomed_land = zoom_int(land_map);
        let zoomed_temp = zoom_float(temp_map);
        let zoomed_rain = zoom_float(rain_map);

        // 256 -> 128 -> 64
        // create hills to height map
        println!("GENERATING TERRAIN AND HEIGHTS");
        let land_64 = zoom_int(zoom_int(zoomed_land));
        let temp_64 = zoom_float(zoom_float(zoomed_temp));
        let rain_64 = zoom_float(zoom_float(zoomed_rain));

        let mut height_map: Vec<Vec<f32>> = land_64.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_height(&mut height_map);

        // 64 -> 32
        // add islands
        println!("ADDING DETAILS");
        let mut land_32 = zoom_int(land_64);
        let temp_32 = zoom_float(temp_64);
        let rain_32 = zoom_float(rain_64);
        let height_32 = zoom_float(height_map);
        add_islands(&mut land_32);

        // 32 -> 16 -> 8
        // TODO: add rivers
        println!("ADDING RIVERS");
        let mut land_8 = zoom_int(zoom_int(land_32));
        let temp_8 = smooth_map(zoom_float(zoom_float(temp_32)));
        let rain_8 = smooth_map(zoom_float(zoom_float(rain_32)));
        let height_8 = smooth_map(zoom_float(zoom_float(height_32)));
        create_rivers(&height_8, &rain_8, &mut land_8, 5);

        

        // 8 -> 4
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
