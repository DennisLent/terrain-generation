use map::Map;
use render::render2d;

mod map;
mod render;

fn main() {
    let map = Map::new(5);
    println!("{:?}", map.land_map);
    render2d(map);
}
