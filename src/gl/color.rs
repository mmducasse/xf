
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgba(rgba: u32) -> Self {
        Self { 
            r: ((rgba >> 24) & 0xFF) as u8, 
            g: ((rgba >> 16) & 0xFF) as u8, 
            b: ((rgba >> 08) & 0xFF) as u8, 
            a: ((rgba >> 00) & 0xFF) as u8 
        }
    }

    pub const BLACK: Color = Color::rgba(0x0000_00FF);
    pub const WHITE: Color = Color::rgba(0xFFFF_FFFF);
}