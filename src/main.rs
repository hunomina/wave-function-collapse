mod map;
mod utils;

use utils::{draw_map, load_tiles};

use macroquad::prelude::*;

const TILES_CONFIG: &str = "tiles.json";
const CELL_WIDTH: f32 = 50.0;
const MAP_SIZE: usize = 20;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Wave Function Collapse"),
        window_width: (MAP_SIZE as f32 * CELL_WIDTH) as i32,
        window_height: (MAP_SIZE as f32 * CELL_WIDTH) as i32,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut map = map::Map::new(MAP_SIZE, load_tiles(String::from(TILES_CONFIG)));

    loop {
        clear_background(BLACK);

        draw_map(&map).await;

        if !map.is_solved() {
            map.collapse_next_cell();
        }

        next_frame().await
    }
}
