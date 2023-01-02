// use crate::{
//     gl::{shader::{Shader, DrawPixel, ShaderEffect}, color::Color, texture::Texture}, 
//     num::{irect::{IRect, rect}, ivec2::IVec2}
// };

// // fn effect(dp: DrawPixel, texture: &Texture) -> DrawPixel {
// //     const REGION: IRect = rect(-1, -1, 3, 3);

// //     let mut dp = dp;
// //     dp.color.r = color.r;
// //     dp.color.g = color.g;
// //     dp.color.b = color.b;

// //     dp
// // }

// fn on_edge(pos: IVec2, texture: &Texture) -> bool {
//     const NEIGHBOR_REGION: IRect = rect(-1, -1, 3, 3);

//     if let Some(color) = texture.get(pos) {
//         if color.a != 0 { return false; }

//         // This pixel is transparent.
//         for offset in NEIGHBOR_REGION.iter() {
//             if let Some(color) = texture.get(pos + offset) {
//                 if color.a != 0 { 
//                     // But it's neighbor is not!
//                     return true; 
//                 }
//             }
//         }
//     }

//     return false;
// }

// pub fn test() {
//     const YELLOW: Color = Color::rgba(0xFFFF_00FF);
//     let outline_effect: ShaderEffect = Box::new(|dp, t| {
//         let mut dp = dp;
//         if on_edge(dp.dst, t) {
//             dp.color = YELLOW;
//         }

//         dp
//     });
//     let outline_shader = Shader::new(outline_effect);
// }