use crate::num::irect::IRect;

use super::shader::Shader;

pub struct DrawParams {
    pub src: Option<IRect>,
    pub shader: Option<Shader>,
}
