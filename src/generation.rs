use rand::Rng;
use rayon::prelude::*;
use std::sync::{Mutex, Arc};
use crate::noise::NoiseGen;
use crate::river::create_rivers;

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


/// generate a map with 2 to 8 proportion of land to water
/// size determines how many pixels there will be (each pixel here is the equivalent to 4096 by 4096 in the final map)
/// a value of 0 will represent water
/// a value of 1 will represent land
/// TODO parallelize
fn generate_islands(size: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();

    let mut board = Vec::new();

    for _ in 0..size {
        let mut row_vector = vec![0; size];
        for i in 0..size {
            let number = rng.gen_range(1..11);
            if number <= 2 {
                row_vector[i] = 1
            } else {
                row_vector[i] = 0
            }
        }
        board.push(row_vector)
    }

    return board;
}

/// function to "zoom" into the board for integers
fn zoom_int(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let new_size = board.len() * 2;
    let mut new_board = vec![vec![0; new_size]; new_size];
    let new_board_mutex = Arc::new(Mutex::new(new_board));

    (0..board.len()).into_par_iter().for_each(|i| {
        (0..board.len()).into_par_iter().for_each(|j| {

            
            let board_value = board[i][j];
            let new_i = i * 2;
            let new_j = j * 2;
            let mut new_board = new_board_mutex.lock().unwrap();
            new_board[new_i][new_j] = board_value;
            new_board[new_i][new_j + 1] = board_value;
            new_board[new_i + 1][new_j] = board_value;
            new_board[new_i + 1][new_j + 1] = board_value;
        });
    });

    Arc::try_unwrap(new_board_mutex).unwrap().into_inner().unwrap()
}

/// function to "zoom" into the board for integers
fn zoom_float(board: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let new_size = board.len() * 2;
    let new_board = vec![vec![0.0; new_size]; new_size];
    let new_board_mutex = Arc::new(Mutex::new(new_board));

    (0..board.len()).into_par_iter().for_each(|i| {
        (0..board.len()).into_par_iter().for_each(|j| {

            
            let board_value = board[i][j];
            let new_i = i * 2;
            let new_j = j * 2;
            let mut new_board = new_board_mutex.lock().unwrap();
            new_board[new_i][new_j] = board_value;
            new_board[new_i][new_j + 1] = board_value;
            new_board[new_i + 1][new_j] = board_value;
            new_board[new_i + 1][new_j + 1] = board_value;
        });
    });

    Arc::try_unwrap(new_board_mutex).unwrap().into_inner().unwrap()
}

///function to add islands in a 2 to 8 ratio as before
/// TODO parallize
fn add_islands(board: &mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();

    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == 0 {
                let number = rng.gen_range(1..11);
                if number <= 2 {
                    board[i][j] = 1
                }
            }
        }
    }
}

/// function used to create the baseline land and ocean generation
/// this is called in the beginning to outline land and ocean in general
fn generate_land_map(size: usize) -> Vec<Vec<i32>> {
    let islands = generate_islands(size);
    let mut zoomed_islands = zoom_int(islands);
    add_islands(&mut zoomed_islands);
    add_islands(&mut zoomed_islands);
    return zoomed_islands;
}

/// helper function to map values to a specfic range
/// simplex gives output in range [-1 to 1]
/// temperatures needs to be mapped [-10, 30]
/// rainfall needs to be mapped [0, 450]
fn map_to_range(value: f32, lower_bound: f32, upper_bound: f32) -> f32{
    lower_bound + (((value - -1.0)*(upper_bound - lower_bound))/(1.0 - -1.0))
}

/// function using simple noise to create temperatures
/// temperatures needs to be mapped [-10, 30]
fn add_temperature(empty_temp: &mut Vec<Vec<f32>>){
    for i in 0..empty_temp.len() {
        for j in 0..empty_temp.len() {
            let value = NoiseGen::noise(i as f32, j as f32);
            empty_temp[i][j] = map_to_range(value, -10.0, 30.0);
        }
    }
}

/// function using simple noise to create rainfall
/// rainfall needs to be mapped [0, 450]
fn add_rainfall(empty_rain: &mut Vec<Vec<f32>>){
    for i in 0..empty_rain.len() {
        for j in 0..empty_rain.len() {
            let value = NoiseGen::noise(i as f32, j as f32);
            empty_rain[i][j] = map_to_range(value, 0.0, 450.0);
        }
    }
}

/// function using simple noise to generate elevation on the map
/// using minecraft as a reference we map height [0, 255] where 65 is sea level for more diversity
fn add_height(empty_height: &mut Vec<Vec<f32>>){
    for i in 0..empty_height.len() {
        for j in 0..empty_height.len() {
            let value = NoiseGen::noise(i as f32, j as f32);
            empty_height[i][j] = map_to_range(value, 0.0, 255.0);
        }
    }
}

/// function to smoothen the noise generated to avoid weird changes in biomes
fn smooth_map(map: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let rows = map.len();
    let cols = map[0].len();
    let mut smoothed_map = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut sum = 0.0;
            let mut count = 0;

            for di in -1..=1 {
                for dj in -1..=1 {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                        sum += map[ni as usize][nj as usize];
                        count += 1;
                    }
                }
            }

            smoothed_map[i][j] = sum / count as f32;
        }
    }

    smoothed_map
}
