use std::rc::Rc;

use macroquad::texture::Texture2D;

#[derive(Clone)]
pub struct Texture {
    inner: Rc<Texture2D>,
}

impl Texture {
    pub fn new(inner: Texture2D) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn inner(&self) -> &Texture2D {
        &self.inner
    }
}
