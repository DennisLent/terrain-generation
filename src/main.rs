use generation::Map;
use render::render2d;

mod gen_utils;
mod generation;
mod render;
mod river;
mod simplex;
mod sines;

fn main() {
    let map = Map::new(1);

    //map.print();

    render2d(map);
}
