#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xff) as u8,
            g: ((rgb >> 8) & 0xff) as u8,
            b: ((rgb) & 0xff) as u8,
        }
    }
}

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub const WHITE: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0xff,
};
