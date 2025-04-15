
use std::collections::HashMap;

use macroquad::{color::Color, shapes::draw_rectangle};

use crate::tile::{PixelShades, Tile};

const SIZE_X: usize = 8;
const SIZE_Y: usize = 8;
const TILE_SIZE: usize = 8;

pub struct EditorGrid {
    start_x: f32,
    start_y: f32,
    scale: f32,
    pixels: Tile,
    palette: HashMap<PixelShades, Color>,
    background: Color
}

impl EditorGrid {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        EditorGrid {
            start_x: x,
            start_y: y,
            scale: EditorGrid::calc_scale(width, height),
            ..Default::default()
        }
    }

    pub fn resize(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.start_x = x;
        self.start_y = y;
        self.scale = EditorGrid::calc_scale(width, height);
    }

    pub fn draw(&self) {
        let (min_width, min_height) = EditorGrid::calc_min_size();

        draw_rectangle(
            self.start_x,
            self.start_y,
            (min_width * self.scale),
            (min_height * self.scale),
            self.background
        );

        for y in 0..SIZE_Y {
            for x in 0..SIZE_X {
                draw_rectangle(
                    self.start_x + self.scale + (x * (TILE_SIZE + 1)) as f32 * self.scale,
                    self.start_y + self.scale + (y * (TILE_SIZE + 1)) as f32 * self.scale,
                    TILE_SIZE as f32 * self.scale,
                    TILE_SIZE as f32 * self.scale,
                    self.palette.get(&self.pixels.get(x, y)).unwrap().to_owned()
                );
            }
        }
    }

    pub fn click(&mut self, posx: f32, posy: f32, shade: PixelShades) {
        let (min_width, min_height) = EditorGrid::calc_min_size();
        if posx > self.start_x && posx < (self.start_x + min_width * self.scale) &&
        posy > self.start_y && posy < (self.start_y + min_height * self.scale) {
            let x = ((posx - self.start_x) / ((TILE_SIZE + 1) as f32 * self.scale)) as usize;
            let y = ((posy - self.start_y) / ((TILE_SIZE + 1) as f32 * self.scale)) as usize;
            self.pixels.set(x,y, shade);
        }
    }

    const fn calc_min_size() -> (f32, f32) {
        let min_width = (SIZE_X * TILE_SIZE + SIZE_X + 1) as f32;
        let min_height = (SIZE_Y * TILE_SIZE + SIZE_Y + 1) as f32;
        (min_width, min_height)
    }

    fn calc_scale(width: f32, height: f32) -> f32 {
        let (min_width, min_height) = EditorGrid::calc_min_size();
        let scale_width = width / min_width;
        let scale_height = height / min_height;
        let mut scale = scale_width.min(scale_height);
        if scale < 1.0 {
            scale = 1.0
        }
        scale
    }
}

impl Default for EditorGrid {
    fn default() -> Self {
        let mut palette = HashMap::new();
        palette.insert(PixelShades::Full, Color::from_rgba(0, 0, 0, 255));
        palette.insert(PixelShades::TwoThirds, Color::from_rgba(85, 85, 85, 255));
        palette.insert(PixelShades::OneThird, Color::from_rgba(170, 170, 170, 255));
        palette.insert(PixelShades::None, Color::from_rgba(255, 255, 255, 255));
        EditorGrid { start_x: 0.0, start_y: 0.0, scale: 1.0, pixels: Tile::new(), palette, background: Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 } }
    }
}