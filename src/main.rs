use generation::Map;
use render::render2d;

mod generation;
mod render;

fn main() {
    let map = Map::new(5);

    render2d(map);
}
