use crate::{gl::texture::Texture, num::ivec2::IVec2};

/// A mapping from char values to a region in a font texture image.
pub trait Font {
    fn char_size(&self) -> IVec2;
    fn lookup(&self, c: char) -> CharData;
    fn texture(&self) -> &Texture;
}

pub struct CharData {
    pub src_pos: IVec2,
    pub draw_offset: IVec2,
}
