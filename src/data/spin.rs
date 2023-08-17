#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Spin {
    Ccw,
    Cw,
}

impl Spin {
    pub fn is_ccw(self) -> bool {
        self == Self::Ccw
    }
}