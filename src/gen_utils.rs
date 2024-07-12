use crate::simplex::SimplexNoise;
use rand::prelude::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// function used to create the baseline land and ocean generation
/// this is called in the beginning to outline land and ocean in general
/// start with a tile that is 4096 later
/// zoom to 2048 and add more islands
pub fn generate_land_map(size: usize) -> Vec<Vec<i32>> {
    let islands = generate_islands(size);
    let mut zoomed_islands = zoom_int(islands);
    add_islands(&mut zoomed_islands);
    add_islands(&mut zoomed_islands);
    return zoomed_islands;
}

/// generate a map with 3 to 7 proportion of land to water
/// size determines how many pixels there will be (each pixel here is the equivalent to 4096 by 4096 in the final map)
/// a value of 0 will represent water
/// a value of 1 will represent land
/// TODO parallelize
pub fn generate_islands(size: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();

    let mut board = Vec::new();

    for _ in 0..size {
        let mut row_vector = vec![0; size];
        for i in 0..size {
            let number = rng.gen_range(1..11);
            if number <= 3 {
                row_vector[i] = 1
            } else {
                row_vector[i] = 0
            }
        }
        board.push(row_vector)
    }

    return board;
}

/// function to "zoom" into the board for integers and add imperfections
/// this is only for landmasses to reduce the amount of straight edges the chance is 50%
pub fn zoom_int(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let new_size = board.len() * 2;
    let new_board = vec![vec![0; new_size]; new_size];
    let new_board_mutex = Arc::new(Mutex::new(new_board));
    (0..board.len()).into_par_iter().for_each(|i| {
        (0..board.len()).into_par_iter().for_each(|j| {
            let board_value = board[i][j];
            let new_i = i * 2;
            let new_j = j * 2;

            let mut new_board = new_board_mutex.lock().unwrap();

            // Original value assignments
            new_board[new_i][new_j] = board_value;
            new_board[new_i][new_j + 1] = board_value;
            new_board[new_i + 1][new_j] = board_value;
            new_board[new_i + 1][new_j + 1] = board_value;

            // Add imperfections
            let mut rng = rand::thread_rng();
            let mut add_variation = |ni: usize, nj: usize| {
                if rng.gen::<f32>() < 0.5 {
                    let mut neighbors = vec![board_value];

                    // Collect valid neighbors
                    if ni > 0 {
                        neighbors.push(new_board[(ni - 1) / 2][nj / 2]);
                    }
                    if nj > 0 {
                        neighbors.push(new_board[ni / 2][(nj - 1) / 2]);
                    }
                    if ni / 2 + 1 < board.len() {
                        neighbors.push(board[ni / 2 + 1][nj / 2]);
                    }
                    if nj / 2 + 1 < board.len() {
                        neighbors.push(board[ni / 2][nj / 2 + 1]);
                    }

                    new_board[ni][nj] = *neighbors.choose(&mut rng).unwrap();
                }
            };

            // Apply variation to each new cell
            add_variation(new_i, new_j);
            add_variation(new_i, new_j + 1);
            add_variation(new_i + 1, new_j);
            add_variation(new_i + 1, new_j + 1);
        });
    });

    Arc::try_unwrap(new_board_mutex)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// function to "zoom" into the board for floats and add imperfections
/// since float is for height, temp and rainfall this chance will be 30%
pub fn zoom_float(board: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let new_size = board.len() * 2;
    let new_board = vec![vec![0.0; new_size]; new_size];
    let new_board_mutex = Arc::new(Mutex::new(new_board));
    (0..board.len()).into_par_iter().for_each(|i| {
        (0..board.len()).into_par_iter().for_each(|j| {
            let board_value = board[i][j];
            let new_i = i * 2;
            let new_j = j * 2;

            let mut new_board = new_board_mutex.lock().unwrap();

            // Original value assignments
            new_board[new_i][new_j] = board_value;
            new_board[new_i][new_j + 1] = board_value;
            new_board[new_i + 1][new_j] = board_value;
            new_board[new_i + 1][new_j + 1] = board_value;

            // Add imperfections
            let mut rng = rand::thread_rng();
            let mut add_variation = |ni: usize, nj: usize| {
                if rng.gen::<f32>() < 0.3 {
                    // 25% chance to change the value
                    let mut neighbors = vec![board_value];

                    // Collect valid neighbors
                    if ni > 0 {
                        neighbors.push(new_board[(ni - 1) / 2][nj / 2]);
                    }
                    if nj > 0 {
                        neighbors.push(new_board[ni / 2][(nj - 1) / 2]);
                    }
                    if ni / 2 + 1 < board.len() {
                        neighbors.push(board[ni / 2 + 1][nj / 2]);
                    }
                    if nj / 2 + 1 < board.len() {
                        neighbors.push(board[ni / 2][nj / 2 + 1]);
                    }

                    new_board[ni][nj] = *neighbors.choose(&mut rng).unwrap();
                }
            };

            // Apply variation to each new cell
            add_variation(new_i, new_j);
            add_variation(new_i, new_j + 1);
            add_variation(new_i + 1, new_j);
            add_variation(new_i + 1, new_j + 1);
        });
    });

    Arc::try_unwrap(new_board_mutex)
        .unwrap()
        .into_inner()
        .unwrap()
}

///function to add islands in a 2 to 8 ratio as before
/// TODO parallize
pub fn add_islands(board: &mut Vec<Vec<i32>>) {
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

/// helper function to map values to a specfic range
/// simplex gives output in range [-1 to 1]
/// temperatures needs to be mapped [-10, 30]
/// rainfall needs to be mapped [0, 450]
pub fn map_to_range(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
    lower_bound + (((value - -1.0) * (upper_bound - lower_bound)) / (1.0 - -1.0))
}

/// function using simple noise to create temperatures
/// temperatures needs to be mapped [-10, 30]
pub fn add_temperature(empty_temp: &mut Vec<Vec<f32>>) {
    for i in 0..empty_temp.len() {
        for j in 0..empty_temp.len() {
            let value = SimplexNoise::noise(i as f32, j as f32);
            empty_temp[i][j] = map_to_range(value, -10.0, 30.0);
        }
    }
}

/// function using simple noise to create rainfall
/// rainfall needs to be mapped [0, 450]
pub fn add_rainfall(empty_rain: &mut Vec<Vec<f32>>) {
    for i in 0..empty_rain.len() {
        for j in 0..empty_rain.len() {
            let value = SimplexNoise::noise(i as f32, j as f32);
            empty_rain[i][j] = map_to_range(value, 0.0, 450.0);
        }
    }
}

/// function using simple noise to generate elevation on the map
/// using minecraft as a reference we map height [0, 255] where 65 is sea level for more diversity
pub fn add_height(empty_height: &mut Vec<Vec<f32>>) {
    for i in 0..empty_height.len() {
        for j in 0..empty_height.len() {
            let value = SimplexNoise::noise(i as f32, j as f32);
            empty_height[i][j] = map_to_range(value, 0.0, 255.0);
        }
    }
}

pub fn add_oceans(land_map: &mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    let height = land_map.len();
    let width = land_map[0].len();

    for i in 0..height {
        for j in 0..width {
            if land_map[i][j] == 0 {
                let mut potential_shore: bool = false;
                let mut potential_deep_ocean: bool = true;

                // Check for adjacent tiles
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if ni >= 0 && ni < height as isize && nj >= 0 && nj < width as isize {
                            let ni = ni as usize;
                            let nj = nj as usize;

                            if land_map[ni][nj] == 1 {
                                potential_shore = true;
                                potential_deep_ocean = false;
                            } else if land_map[ni][nj] == 0 {
                                potential_shore = false;
                            }
                        }
                    }
                }

                if potential_shore && rng.gen::<f32>() < 0.5 {
                    land_map[i][j] = 2; // Shallow ocean
                }

                if potential_deep_ocean && rng.gen::<f32>() < 0.9 {
                    land_map[i][j] = -1; // Deep ocean
                }
            }
        }
    }
}

/// function to smoothen the noise generated to avoid weird changes in biomes
pub fn smooth_map(map: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
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
