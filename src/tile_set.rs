use std::{
    fs,
    io::Error,
    path::{self, Path},
};

use regex::Regex;

use crate::tile::Tile;

pub struct TileSet {
    tiles: Vec<Tile>,
}

impl Default for TileSet {
    fn default() -> Self {
        TileSet {
            tiles: vec![Tile::new()],
        }
    }
}

impl TileSet {
    pub fn fromFile(filename: &str) -> Result<Self, Error> {
        if let Ok(contents) = fs::read_to_string(filename) {
            if let Ok(re) = Regex::new(r"const unsigned char ([a-zA-Z0-9-_]+)\[\] =.\{\n?(.*)\n?};")
            {
                for c in re.captures_iter(&contents) {
                    for group in c.iter() {
                        if let Some(g) = group {
                            println!("{:?}", g);
                        }
                    }
                }
            }
        }
        Err(Error::new(std::io::ErrorKind::Other, "Could not read file"))
    }
}
