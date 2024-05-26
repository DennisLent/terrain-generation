use generation::Map;
use render::render2d;

mod generation;
mod render;
mod sines;
mod river;
mod gen_utils;
mod simplex;

fn main() {
    let map = Map::new(2);

    //map.print();

    render2d(map);
}
