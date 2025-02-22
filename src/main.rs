mod map;
mod utils;

use utils::{build_map, draw_error_message, draw_map, load_tiles};

use macroquad::prelude::*;

const TILES_CONFIG: &str = "tiles.json";
const MAP_SIZE: usize = 20;
const MAX_WINDOW_SIZE: i32 = 1000;
const CELL_WIDTH: f32 = (MAX_WINDOW_SIZE / MAP_SIZE as i32) as f32;

const ERROR_MESSAGE: &str = "Error while collapsing the map. Press [R] to restart.";

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
    let (tiles, textures) = load_tiles(String::from(TILES_CONFIG)).await;
    let mut collapse_failed = false;
    let mut map = build_map(&tiles);

    loop {
        clear_background(BLACK);

        if collapse_failed == false {
            if !map.is_solved() {
                if let Err(_) = map.collapse_next_cell() {
                    collapse_failed = true;
                }
            }

            draw_map(&map, &textures).await;
        } else {
            draw_error_message(ERROR_MESSAGE);
        }

        if is_key_down(KeyCode::R) {
            map = build_map(&tiles);
            collapse_failed = false;
        }

        next_frame().await
    }
}
