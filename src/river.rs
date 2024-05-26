use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rayon::prelude::*;
use rand::prelude::SliceRandom;

/// struct to hold information about a river
struct River{
    start_point: (usize, usize),
    end_point: (usize, usize)
}

/// function to find proper starting points for rivers
/// a river should start in a high elevation height > 150
/// and good rainfall precipitaion > 200
fn find_river_starting_points(height_map: &Vec<Vec<f32>>, rain_map: &Vec<Vec<f32>>) -> Vec<(usize, usize)>{
    let starting_points: Vec<(usize, usize)> = (0..height_map.len()).into_par_iter().flat_map(|i| {
        (0..height_map[i].len()).into_par_iter().filter_map(move |j| {
            if height_map[i][j] > 150.0 && rain_map[i][j] > 200.0 {
                Some((i, j))
            } else {
                None
            }
        })
    }).collect();

    starting_points
}

/// function to find an ending point for the river
/// a river can end in the ocean land_map == 0
/// or in a lake height_map < 65
/// need to find the closest point using BFS
// TODO: change it to A* to save on time?
fn find_river_end(start: (usize, usize), height_map: &Vec<Vec<f32>>, land_map: &Vec<Vec<i32>>) -> Option<(usize, usize)>{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if land_map[current.0][current.1] == 0 || height_map[current.0][current.1] < 65.0 {
            return Some(current);
        }
        // check adjacent points
        for &neighbor in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_point = ((current.0 as isize + neighbor.0) as usize, (current.1 as isize + neighbor.1) as usize);

            if !visited.contains(&new_point) && new_point.0 < land_map.len() && new_point.1 < land_map[0].len(){
                visited.insert(new_point);
                queue.push_back(new_point);
            }
        }
    }

    None
}

/// helper function to filter out points that are too close to eachother
/// since there are a lot of valid river starting points we need to cut down on them
fn filter_points(points: Vec<(usize, usize)>, min_distance: usize) -> Vec<(usize, usize)> {
    let mut filtered_points = Vec::new();
    let mut visited = HashSet::new();

    for &(x, y) in &points {
        let mut too_close = false;
        for i in 0..=min_distance {
            for j in 0..=min_distance {
                if visited.contains(&(x.wrapping_sub(i), y.wrapping_sub(j))) ||
                   visited.contains(&(x.wrapping_add(i), y.wrapping_sub(j))) ||
                   visited.contains(&(x.wrapping_sub(i), y.wrapping_add(j))) ||
                   visited.contains(&(x.wrapping_add(i), y.wrapping_add(j))) {
                    too_close = true;
                    break;
                }
            }
            if too_close {
                break;
            }
        }
        if !too_close {
            filtered_points.push((x, y));
            visited.insert((x, y));
        }
    }

    filtered_points
}


/// Function to find a path from start to end using BFS
/// We add a height constraint to make sure that we go from a high altitude to a low altitude
fn find_path(start: (usize, usize), end: (usize, usize), height_map: &Vec<Vec<f32>>) -> Option<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut came_from = vec![vec![None; height_map[0].len()]; height_map.len()];
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = vec![current];
            let mut prev = current;
            while let Some(p) = came_from[prev.0][prev.1] {
                path.push(p);
                prev = p;
            }
            path.reverse();
            return Some(path);
        }

        let mut neighbors = directions.to_vec();
        neighbors.shuffle(&mut rng); // Randomize the order of exploration

        for &(dx, dy) in &neighbors {
            let new_point = ((current.0 as isize + dx) as usize, (current.1 as isize + dy) as usize);
            if new_point.0 < height_map.len() && new_point.1 < height_map[0].len() && !visited.contains(&new_point) {
                if height_map[new_point.0][new_point.1] <= height_map[current.0][current.1] {
                    visited.insert(new_point);
                    queue.push_back(new_point);
                    came_from[new_point.0][new_point.1] = Some(current);
                }
            }
        }
    }

    None
}

/// main function called to create the rivers and update the land_map
pub fn create_rivers(height_map: &Vec<Vec<f32>>, rain_map: &Vec<Vec<f32>>, land_map: &mut Vec<Vec<i32>>, min_distance: usize) {
    let starting_points = find_river_starting_points(height_map, rain_map);
    let filtered_points = filter_points(starting_points, min_distance);

    for point in filtered_points {
        if let Some(end) = find_river_end(point, height_map, land_map) {
            if let Some(path) = find_path(point, end, height_map) {
                // Update the land_map to reflect the river path
                for &(x, y) in &path {
                    land_map[x][y] = 0;
                }
            }
        }
    }
}

// just for checking not needed actually
// pub fn test(height_map: &Vec<Vec<f32>>, rain_map: &Vec<Vec<f32>>, land_map: &Vec<Vec<i32>>){
//     let starting_points = filter_points(find_river_starting_points(height_map, rain_map), 20);

//     for point in starting_points{
//         let end = find_river_end(point, height_map, land_map);
//         match end{
//             Some(end_point) => {
//                 println!("({}, {}) ->  ({}, {})", point.0, point.1, end_point.0, end_point.1);
//             }
//             _ => {
//                 println!("no connection possible")
//             }
//         }
//     }
// }


