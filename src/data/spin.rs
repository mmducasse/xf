#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Spin {
    Ccw,
    Cw,
}

impl Spin {
    pub fn is_ccw(self) -> bool {
        self == Self::Ccw
    }

    pub fn opposite(self) -> Self {
        match self {
            Spin::Ccw => Spin::Cw,
            Spin::Cw => Spin::Ccw,
        }
    }
}
