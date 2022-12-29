use std::ops::{AddAssign, SubAssign, Add, Sub};

#[derive(Clone, Copy)]
pub struct Limit<T> {
    pub min: T,
    pub max: T,
    pub value: T,
}

impl<T> Limit<T>
where T: Copy {
    pub const fn new(min: T, max: T, value: T) -> Self {
        Self { min, max, value, }
    }

    pub const fn new_min(min: T, max: T) -> Self {
        Self { min, max, value: min, }
    }

    pub const fn new_max(min: T, max: T) -> Self {
        Self { min, max, value: max, }
    }
}

impl<T> AddAssign<T> for Limit<T>
where T: Add<T, Output = T> + Ord + Copy
{
    fn add_assign(&mut self, rhs: T) {
        self.value = T::min(self.max, self.value + rhs);
    }
}

impl<T> SubAssign<T> for Limit<T>
where T: Sub<T, Output = T> + Ord + Copy
{
    fn sub_assign(&mut self, rhs: T) {
        self.value = T::max(self.min, self.value - rhs);
    }
}

impl<T> PartialEq<T> for Limit<T>
where T: PartialEq {
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}