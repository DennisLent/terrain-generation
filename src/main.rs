use generation::Map;
use render::render2d;

mod generation;
mod render;
mod noise;

fn main() {
    let map = Map::new(5);

    println!("tiles in map: {}", map.board_size());

    map.print();

    render2d(map);
}
