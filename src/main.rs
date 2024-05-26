use generation::Map;
use render::render2d;

mod generation;
mod render;
mod noise;

fn main() {
    let map = Map::new(10);

    map.print();

    render2d(map);
}
