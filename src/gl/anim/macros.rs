// // todo: delete test()
// fn test() {
//     let x= {
//         use crate::num::ivec2::IVec2;
//         use crate::gl::anim::{Seq, seq_row};

//         const LEN: usize = 0;
//         const C: Seq<LEN> = seq_row::<LEN>(0.0, IVec2::ZERO, false);
//         &C
//     };

//     let y= {
//         use crate::num::ivec2::IVec2;
//         use crate::gl::anim::{Seq, seq};

//         //const arr: [IVec2; 1] = [IVec2::ZERO];
//         const LEN: usize = 1;
//         const C: Seq<LEN> = seq::<LEN>(0.0, [IVec2::ZERO], false)
//             .with_offsets([IVec2::ZERO]);
//         &C
//     };

//     use crate::data::dir4::Dir4;
//     use crate::num::ivec2::i2;
//     let dir = Dir4::N;
//     let len = 2;

//     let anim_1 = crate::row!(2, 0.2, i2(2, 2));
//     let anim_4 = crate::row_4!(dir, 2, 0.3, i2(2, 2));

//     let loop_1 = crate::row_l!(2, 0.2, i2(2, 2));
//     let loop_4 = crate::row_4_l!(dir, 2, 0.3, i2(2, 2));
// }

#[macro_export]
macro_rules! seq {
    ($len:expr, $dur:expr, $pts:expr, $loops:expr) => {{
        use crate::gl::animation::{seq, Seq};

        const LEN: usize = $len;
        const C: Seq<LEN> = seq::<LEN>($dur, $pts, $loops);
        &C
    }};
}

#[macro_export]
macro_rules! seq_o {
    ($len:expr, $dur:expr, $pts:expr, $offsets:expr, $loops:expr) => {{
        use crate::gl::animation::{seq, Seq};

        const LEN: usize = $len;
        const C: Seq<LEN> = seq::<LEN>($dur, $pts, $loops).with_offsets($offsets);
        &C
    }};
}

#[macro_export]
macro_rules! row {
    ($len:expr, $dur:expr, $origin:expr) => {{
        use crate::gl::anim::{seq_row, Seq};

        const LEN: usize = $len;
        const C: Seq<LEN> = seq_row::<LEN>($dur, $origin, false);
        &C
    }};
}

#[macro_export]
macro_rules! row_4 {
    ($dir:ident, $len:expr, $dur:expr, $origin:expr) => {{
        use crate::data::dir4::Dir4;
        use crate::num::ivec2::i2;

        match $dir {
            Dir4::N => crate::row!($len, $dur, i2($origin.x, $origin.y + 0)),
            Dir4::E => crate::row!($len, $dur, i2($origin.x, $origin.y + 1)),
            Dir4::S => crate::row!($len, $dur, i2($origin.x, $origin.y + 2)),
            Dir4::W => crate::row!($len, $dur, i2($origin.x, $origin.y + 3)),
        }
    }};
}

#[macro_export]
macro_rules! row_l {
    ($len:expr, $dur:expr, $origin:expr) => {{
        use crate::gl::anim::{seq_row, Seq};

        const LEN: usize = $len;
        const C: Seq<LEN> = seq_row::<LEN>($dur, $origin, true);
        &C
    }};
}

#[macro_export]
macro_rules! row_4_l {
    ($dir:ident, $len:expr, $dur:expr, $origin:expr) => {{
        use crate::data::dir4::Dir4;
        use crate::num::ivec2::i2;

        match $dir {
            Dir4::N => crate::row_l!($len, $dur, i2($origin.x, $origin.y + 0)),
            Dir4::E => crate::row_l!($len, $dur, i2($origin.x, $origin.y + 1)),
            Dir4::S => crate::row_l!($len, $dur, i2($origin.x, $origin.y + 2)),
            Dir4::W => crate::row_l!($len, $dur, i2($origin.x, $origin.y + 3)),
        }
    }};
}
