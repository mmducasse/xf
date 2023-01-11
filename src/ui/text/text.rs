use crate::{
    num::{ivec2::IVec2, irect::ir}, 
    gl::bitmap::Bitmap
};

use super::font::Font;


/// Draws text with the given font to the target bitmap.
pub fn draw(s: &str, font: &dyn Font, width: Option<i32>, org: IVec2, target: &mut dyn Bitmap) {
    let mut grid_pos = IVec2::ZERO;

    for c in s.chars() {
        let char_data = font.lookup(c);
        let dst_pos = (grid_pos * font.char_size()) + char_data.draw_offset + org;

        let src = ir(char_data.src_pos, font.char_size());

        target.draw_texture(font.texture(), src, dst_pos);

        // Increment draw position.
        grid_pos.x += 1;
        if let Some(width) = width {
            if grid_pos.x >= width {
                grid_pos.x = 0;
                grid_pos.y += 1;
            }
        }
    }
}