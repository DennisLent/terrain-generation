use rand::Rng;

//resources
// https://www.alanzucconi.com/2022/06/05/minecraft-world-generation/
// https://www.youtube.com/watch?v=YyVAaJqYAfE&t=1550s
#[derive(Debug)]
pub struct Map {
    pub size: usize,
    pub land_map: Vec<Vec<i32>>,
    pub height_map: Vec<Vec<i32>>,
    pub temperature_map: Vec<Vec<i32>>,
    pub rainfall_map: Vec<Vec<i32>>,
}

impl Map {
    // debugging don't mind this
    fn print_land_map(&self) {
        for row in &self.land_map {
            println!("{:?}", row);
        }
    }

    // main method to be called when generating a map
    pub fn new(size: usize) -> Self {
        let land_map = generate_land_map(size);
        Map {
            size: size,
            land_map: land_map,
            height_map: Vec::new(),
            temperature_map: Vec::new(),
            rainfall_map: Vec::new(),
        }
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

// method used to create the baseline land and ocean generation
// this is called in the beginning to outline land and ocean in general
fn generate_land_map(size: usize) -> Vec<Vec<i32>> {
    let islands = generate_islands(size);
    let mut zoomed_islands = zoom(islands);
    add_islands(&mut zoomed_islands);
    add_islands(&mut zoomed_islands);
    return zoomed_islands;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::new(3);

        map.print_land_map();
    }
}
