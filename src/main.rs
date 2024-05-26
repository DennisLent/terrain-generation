use generation::Map;
use render::render2d;

mod generation;
mod render;
mod noise;
mod river;

fn main() {
    let map = Map::new(2);

    //map.print();

    render2d(map);
}
