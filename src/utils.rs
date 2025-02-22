use macroquad::{
    color::*,
    math::vec2,
    shapes::draw_rectangle_lines,
    text::{draw_text, measure_text},
    texture::{draw_texture_ex, load_image, DrawTextureParams, Texture2D},
    window::{screen_height, screen_width},
};
use serde_json::Value;

use core::f64;
use std::collections::HashMap;

use crate::{
    map::{
        cell::{CellValue, Ports},
        Map,
    },
    CELL_WIDTH, MAP_SIZE,
};

pub async fn draw_map(map: &Map, textures: &HashMap<String, Texture2D>) {
    for line in 0..map.size {
        for column in 0..map.size {
            let x = column as f32 * CELL_WIDTH;
            let y = line as f32 * CELL_WIDTH;
            let cell = map.get_cell(line, column).unwrap();
            if cell.collapsed {
                let cell_value = cell.value().unwrap();
                let texture = textures.get(&cell_value.file).unwrap();
                draw_texture_ex(
                    texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(CELL_WIDTH, CELL_WIDTH)),
                        rotation: match cell_value.image_rotation {
                            3 => -f64::consts::FRAC_PI_2 as f32,
                            2 => f64::consts::PI as f32,
                            1 => (f64::consts::PI / 2.0) as f32,
                            _ => 0.0,
                        },
                        ..DrawTextureParams::default()
                    },
                );
            } else {
                draw_rectangle_lines(x, y, CELL_WIDTH, CELL_WIDTH, 3.0, WHITE);
            }
        }
    }
}

pub async fn load_tiles(file: String) -> (Vec<CellValue>, HashMap<String, Texture2D>) {
    let file_content = std::fs::read_to_string(file).expect("Unable to read tiles file");
    let content: Value = serde_json::from_str(&file_content).expect("Unable to parse tiles file");

    let mut tiles = vec![];
    let mut textures = HashMap::new();

    for tile in content.as_array().unwrap() {
        let file = tile["file"].as_str().unwrap().to_string();

        let image = load_image(&file)
            .await
            .unwrap_or_else(|_| panic!("Impossible to load image {}", file));
        let texture = Texture2D::from_image(&image);
        textures.insert(file.clone(), texture);

        let ports: Vec<Vec<usize>> = tile["ports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|port| {
                port.as_array()
                    .unwrap()
                    .iter()
                    .map(|port| port.as_u64().unwrap() as usize)
                    .collect()
            })
            .collect();
        assert!(ports.len() == 4, "ports length must be 4");

        let rotations = tile["rotations"].as_u64().unwrap() as usize;

        let possible_rotations = (0..rotations).map(|rotation| {
            let mut ports = ports.clone();
            let mut ports = Ports::new(
                ports.remove(0),
                ports.remove(0),
                ports.remove(0),
                ports.remove(0),
            );
            for _ in 0..rotation {
                ports.rotate();
            }
            CellValue::new(file.clone(), ports, rotation)
        });

        tiles.extend(possible_rotations);
    }

    (tiles, textures)
}

pub fn build_map(tiles: &Vec<CellValue>) -> Map {
    Map::new(MAP_SIZE, tiles.clone())
}

pub fn draw_error_message(message: &str) {
    let text_size = measure_text(message, None, 30, 1.0);
    draw_text(
        message,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        30.0,
        WHITE,
    );
}
