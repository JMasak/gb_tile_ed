#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PixelShades {
    None,
    OneThird,
    TwoThirds,
    Full
}

impl PixelShades {
    pub fn msb(&self) -> bool {
        match self {
            PixelShades::None => false,
            PixelShades::OneThird => false,
            PixelShades::TwoThirds => true,
            PixelShades::Full => true
        }
    }

    pub fn lsb(&self) -> bool {
        match self {
            PixelShades::None => false,
            PixelShades::OneThird => true,
            PixelShades::TwoThirds => false,
            PixelShades::Full => true
        }
    }
}

impl From<u8> for PixelShades {
    fn from(value: u8) -> Self {
        match value {
            0 => PixelShades::None,
            1 => PixelShades::OneThird,
            2 => PixelShades::TwoThirds,
            _ => PixelShades::Full
        }
    }
}

impl Into<u8> for PixelShades {
    fn into(self) -> u8 {
        match self {
            PixelShades::None => 0,
            PixelShades::OneThird => 1,
            PixelShades::TwoThirds => 2,
            PixelShades::Full => 3
        }
    }
}

pub struct Tile {
    pixels: [PixelShades; 64]
}

impl Tile {
    pub fn new() -> Self {
        Tile { pixels: [PixelShades::None; 64] }
    }

    pub fn set(&mut self, col: usize, row: usize, shade: PixelShades) {
        self.pixels[row*8+col] = shade;
    }

    pub fn get(&self, col: usize, row: usize) -> PixelShades {
        self.pixels[row*8+col]
    }
}