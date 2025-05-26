use macroquad::{input, prelude::*};

mod editor_grid;
mod tile;
mod tile_set;

// Regex for parsing tiles: r"const unsigned char ([a-zA-Z0-9-_]+)\[\] =.\{\n?(.*)\n?};"gms

#[macroquad::main("GB Tile EDitor")]
async fn main() {
    let mut width = screen_width();
    let mut height = screen_height();
    let mut editor = editor_grid::EditorGrid::new(0.0, 0.0, width, height);

    loop {
        clear_background(BLACK);

        if screen_width() != width || screen_height() != height {
            width = screen_width();
            height = screen_height();
            editor.resize(0.0, 0.0, width, height);
        }

        editor.draw();

        if input::is_mouse_button_pressed(input::MouseButton::Left) {
            let (posx, posy) = input::mouse_position();
            editor.click(posx, posy, tile::PixelShades::OneThird);
        }

        next_frame().await
    }
}
