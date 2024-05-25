use rand::Rng;
use rayon::prelude::*;
use crate::noise::SimplexNoise;

//resources
// https://www.alanzucconi.com/2022/06/05/minecraft-world-generation/
// https://www.youtube.com/watch?v=YyVAaJqYAfE&t=1550s
#[derive(Debug)]
pub struct Map {
    pub land_map: Vec<Vec<i32>>,
    pub height_map: Vec<Vec<i32>>,
    pub temperature_map: Vec<Vec<f32>>,
    pub rainfall_map: Vec<Vec<f32>>,
}

impl Map {
    // debugging don't mind this
    pub fn print(&self) {
        let map_size = self.land_map.len() as usize;

        println!("Land_val, temp, rain");
    
        for i in 0..map_size {
            for j in 0..map_size {
                println!("{} {} {}", self.land_map[i][j], self.temperature_map[i][j], self.rainfall_map[i][j]);
            }
        }
    }

    // main method to be called when generating a map
    pub fn new(size: usize) -> Self {

        //create land map 4096 -> 1024
        let land_map = generate_land_map(size);

        //zoom once more 1024 -> 512
        let zoomed_land_map = zoom(land_map);

        //add temperates and rainfall using noise
        let mut temp_map: Vec<Vec<f32>> = zoomed_land_map.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_temperature(&mut temp_map);
        let mut rain_map: Vec<Vec<f32>> = zoomed_land_map.par_iter().map(|row| vec![0.0; row.len()]).collect();
        add_rainfall(&mut rain_map);

        Map {
            land_map: zoomed_land_map,
            height_map: Vec::new(),
            temperature_map: temp_map,
            rainfall_map: rain_map,
        }
    }

    pub fn board_size(&self) -> usize{
        self.land_map.len()*self.land_map.len()
    }
}


// generate a map with 2 to 8 proportion of land to water
// size determines how many pixels there will be (each pixel here is the equivalent to 4096 by 4096 in the final map)
// a value of 0 will represent water
// a value of 1 will represent land
// TODO parallelize
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

// function to "zoom" into the board
// TODO parallize
fn zoom(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let new_size = board.len() * 2;
    let mut new_board = vec![vec![0; new_size]; new_size];

    for i in 0..board.len() {
        for j in 0..board.len() {
            let board_value = board[i][j];
            let new_i = i * 2;
            let new_j = j * 2;
            new_board[new_i][new_j] = board_value;
            new_board[new_i][new_j + 1] = board_value;
            new_board[new_i + 1][new_j] = board_value;
            new_board[new_i + 1][new_j + 1] = board_value;
        }
    }

    return new_board;
}

//function to add islands in a 2 to 8 ratio as before
// TODO parallize
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

// function used to create the baseline land and ocean generation
// this is called in the beginning to outline land and ocean in general
fn generate_land_map(size: usize) -> Vec<Vec<i32>> {
    let islands = generate_islands(size);
    let mut zoomed_islands = zoom(islands);
    add_islands(&mut zoomed_islands);
    add_islands(&mut zoomed_islands);
    return zoomed_islands;
}

// helper function to map values to a specfic range
// simplex gives output in range [-1 to 1]
// temperatures needs to be mapped [-10, 30]
// rainfall needs to be mapped [0, 450]
fn map_to_range(value: f32, lower_bound: f32, upper_bound: f32) -> f32{
    lower_bound + (((value - -1.0)*(upper_bound - lower_bound))/(1.0 - -1.0))
}

// function using simplex noise to create temperatures
// temperatures needs to be mapped [-10, 30]
fn add_temperature(empty_temp: &mut Vec<Vec<f32>>){
    for i in 0..empty_temp.len() {
        for j in 0..empty_temp.len() {
            let value = SimplexNoise::noise(i as f32, j as f32);
            empty_temp[i][j] = map_to_range(value, -10.0, 30.0);
        }
    }
}

// function using simplex noise to create rainfall
// rainfall needs to be mapped [0, 450]
fn add_rainfall(empty_rain: &mut Vec<Vec<f32>>){
    for i in 0..empty_rain.len() {
        for j in 0..empty_rain.len() {
            let value = SimplexNoise::noise(i as f32, j as f32);
            empty_rain[i][j] = map_to_range(value, 0.0, 450.0);
        }
    }
}
