use pixels::{Pixels as RawPixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use crate::num::ivec2::IVec2;

use super::pixels::Pixels;

pub struct Ui {
    pub window: Window,
    pub pixels: Pixels,
}

impl Ui {
    pub fn new(title: &str, scale: f64, size: IVec2, event_loop: &EventLoop<()>) -> Self {
        let window = {
            let l_size = LogicalSize::new(size.x as f64, size.y as f64);
            let scaled_size =
                LogicalSize::new(l_size.width as f64 * scale, l_size.height as f64 * scale);
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(scaled_size)
                .with_min_inner_size(l_size)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let w_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(w_size.width, w_size.height, &window);
            let raw_pixels = RawPixels::new(size.x as u32, size.y as u32, surface_texture).unwrap();
            Pixels::new(raw_pixels, size)
        };

        Self { window, pixels }
    }
}
