use crate::{
    gl::{
        bitmap::{draw_rect_default, Bitmap},
        color::Color,
        draw_params::DrawParams,
        texture::Texture,
    },
    num::{
        irect::{ir, IRect},
        ivec2::{i2, IVec2},
    },
};
use pixels::{Error, Pixels as RawPixels};

const BYTE_PER_PIXEL: usize = 4;

/// Wrapper for Pixels type that implements Bitmap.
pub struct Pixels {
    raw_pixels: RawPixels,
    size: IVec2,
}

impl Pixels {
    pub fn new(pixels: RawPixels, size: IVec2) -> Self {
        Self {
            raw_pixels: pixels,
            size,
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        self.raw_pixels.render()
    }
}

impl Bitmap for Pixels {
    fn size(&self) -> IVec2 {
        self.size
    }

    fn get_pixel(&self, pos: IVec2) -> Color {
        let idx = (((pos.y * self.size.x) + pos.x) * 4) as usize;
        let f = self.raw_pixels.get_frame();
        Color {
            r: f[idx + 0],
            g: f[idx + 1],
            b: f[idx + 2],
            a: f[idx + 3],
        }
    }

    unsafe fn set_pixel(&mut self, pos: IVec2, color: Color) {
        let idx = (((pos.y * self.size.x) + pos.x) * 4) as usize;
        let f = self.raw_pixels.get_frame_mut();

        f[idx + 0] = color.r;
        f[idx + 1] = color.g;
        f[idx + 2] = color.b;
        f[idx + 3] = color.a;
    }

    fn draw_texture_x(&mut self, texture: &Texture, dst_pt: IVec2, params: DrawParams) {
        draw_texture_x_custom(self, texture, dst_pt, params);
    }

    fn draw_rect(&mut self, rect: IRect, color: Color) {
        draw_rect_default(self, rect, color);
    }
}

#[inline]
fn draw_texture_x_custom(
    pixels: &mut Pixels,
    texture: &Texture,
    dst_pt: IVec2,
    params: DrawParams,
) {
    let src = params.src.unwrap_or(texture.bounds());
    let offset = dst_pt - src.pos;

    let Some(dst) =
        ir(dst_pt, src.size)
        .intersection(IRect::of_size(pixels.size())) else
    {
        return;
    };

    let src = ir(dst.pos - offset, dst.size);

    let pixels_size = pixels.size();

    let src_data = texture.data();
    let dst_data = pixels.raw_pixels.get_frame_mut();

    for y in 0..src.h() {
        let src_start_idx = IVec2::unwrap(i2(0, y) + src.pos, texture.size().x) as usize;
        let dst_start_idx =
            IVec2::unwrap(i2(0, y) + dst.pos, pixels_size.x) as usize * BYTE_PER_PIXEL;

        for x in 0..src.w() as usize {
            let src_idx = src_start_idx + x;
            let dst_idx = dst_start_idx + x * BYTE_PER_PIXEL;
            let color = src_data[src_idx];
            if color.a < 10 {
                continue;
            }

            dst_data[dst_idx + 0] = color.r;
            dst_data[dst_idx + 1] = color.g;
            dst_data[dst_idx + 2] = color.b;
            dst_data[dst_idx + 3] = color.a;
        }
    }
}
