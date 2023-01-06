
use super::ivec2::{IVec2, i2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IRect {
    pub pos: IVec2,
    pub size: IVec2,
}

/// Creates an `IRect`.
pub const fn ir(pos: IVec2, size: IVec2) -> IRect {
    IRect { pos, size }
}

/// Creates an `IRect`.
pub const fn rect(x: i32, y: i32, w: i32, h: i32) -> IRect {
    IRect { pos: i2(x, y), size: i2(w, h) }
}

impl IRect {
    /// Creates a rectangle at `(0, 0)` with the given size.
    pub fn of_size(size: IVec2) -> Self {
        Self { pos: IVec2::ZERO, size }
    }

    /// The rectangle's x position (top-left origin).
    #[inline]
    pub fn x(&self) -> i32 { self.pos.x }
    
    /// The rectangle's y position (top-left origin).
    #[inline]
    pub fn y(&self) -> i32 { self.pos.y }

    /// The rectangle's width.
    #[inline]
    pub fn w(&self) -> i32 { self.size.x }

    /// The rectangle's width.
    #[inline]
    pub fn h(&self) -> i32 { self.size.y }

    /// The rectangle's top-row y position.
    #[inline]
    pub fn top(&self) -> i32 { self.y() }

    /// The rectangle's bottom-row y position.
    #[inline]
    pub fn bottom(&self) -> i32 { self.y() + self.h() - 1 }

    /// The rectangle's left-column x position.
    #[inline]
    pub fn left(&self) -> i32 { self.x() }

    /// The rectangle's right-column x position.
    #[inline]
    pub fn right(&self) -> i32 { self.x() + self.w() - 1 }

    /// Does the rectangle contain the point `pt`?
    pub fn contains(&self, pt: IVec2) -> bool {
        (self.left() <= pt.x) && 
        (self.right() >= pt.x) &&
        (self.top() <= pt.y) && 
        (self.bottom() >= pt.y)
    }

    /// Does the rectangle overlap the other rectangle?
    pub fn overlaps(&self, other: IRect) -> bool {
        self.left() <= other.right() &&
        self.right() >= other.left() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
    }

    /// Gets the intersection of the two rectangles, if they overlap.
    pub fn intersection(&self, other: IRect) -> Option<IRect> {
        let left = self.left().max(other.left());
        let top = self.top().max(other.top());
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        if right < left || bottom < top {
            return None;
        }

        Some(rect(left, top, right + 1 - left, bottom + 1 - top))
    }

    /// Gets the union of the two rectangles.
    pub fn union(&self, other: IRect) -> IRect {
        let left = self.left().min(other.left());
        let top = self.top().min(other.top());
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        rect(left, top, right + 1 - left, bottom + 1 - top)
    }

    pub fn iter<'a>(&'a self) -> IRectIter<'a> {
        IRectIter { rect: self, curr: self.pos }
    }
}

pub struct IRectIter<'a> {
    rect: &'a IRect,
    curr: IVec2,
}

impl<'a> Iterator for IRectIter<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<IVec2> {
        let curr = self.curr;

        if !self.rect.contains(curr) {
            return None;
        }

        let mut next = i2(curr.x + 1, curr.y);
        if next.x > self.rect.right() {
            next = i2(self.rect.left(), curr.y + 1)
        }

        self.curr = next;
        Some(curr)
    }
}