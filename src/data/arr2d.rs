use std::ops::{Index, IndexMut};

use crate::num::{ivec2::{IVec2, i2}, irect::{IRect, ir}};


pub struct Arr2D<T> {
    data: Vec<T>,
    size: IVec2,
}

impl<T> Arr2D<T> {
    pub fn new(data: Vec<T>, size: IVec2) -> Self {
        assert!((data.len() as i32) == (size.x * size.y));

        Self {
            data,
            size,
        }
    }

    pub fn get(&self, pt: IVec2) -> Option<&T> {
        let i = self.to_idx(pt);
        self.get_i(i)
    }

    pub fn get_mut(&mut self, pt: IVec2) -> Option<&mut T> {
        let i = self.to_idx(pt);
        self.get_i_mut(i)
    }

    pub fn get_i(&self, idx: usize) -> Option<&T> {
        if idx < self.data.len() {
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn get_i_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.data.len() {
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, pt: IVec2, value: T) {
        let idx = self.to_idx(pt);
        self.set_i(idx, value);
    }

    pub fn set_i(&mut self, idx: usize, value: T) {
        self.data[idx] = value;
    }

    pub fn size(&self) -> IVec2 {
        self.size
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn bounds(&self) -> IRect {
        ir(IVec2::splat(0), self.size())
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn swap(&mut self, p1: IVec2, p2: IVec2) {
        let i1 = self.to_idx(p1);
        let i2 = self.to_idx(p2);

        self.data.swap(i1, i2);
    }

    pub fn to_pt(&self, i: usize) -> IVec2 {
        let i = i as i32;
        i2(i % self.size.x, i / self.size.x)
    }

    pub fn to_idx(&self, pt: IVec2) -> usize {
        (pt.x + pt.y * self.size.x) as usize
    }

    pub fn iter(&self) -> Arr2DIter<'_, T> {
        Arr2DIter { arr2d: self, curr_pt: i2(0, 0) }
    }
}

impl<T: Clone> Arr2D<T> {
    pub fn default(default_value: T, size: IVec2) -> Self {
        let data = vec![default_value; size.product() as usize];
        
        Self {
            data,
            size,
        }
    }
}

impl<T: Copy> Arr2D<T> {
    pub fn copy_area(&self, area: IRect) -> Arr2D<T> {
        let mut copy = Arr2D::default(self[area.pos], area.size);

        for dst_pt in copy.bounds().iter() {
            let src_pt = area.pos + dst_pt;
            let value = self[src_pt];
            copy[dst_pt] = value;
        }

        copy
    }

    pub fn set_area(&mut self, origin: IVec2, other: Arr2D<T>) {
        for src_pt in other.bounds().iter() {
            let dst_pt = src_pt + origin;
            if self.bounds().contains(dst_pt) {
                self[dst_pt] = other[src_pt];
            }
        }
    }
}

pub struct Arr2DIter<'a, T> {
    arr2d: &'a Arr2D<T>,
    curr_pt: IVec2,
}

impl<'a, T> Iterator for Arr2DIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_pt = self.curr_pt;
        let idx = self.arr2d.to_idx(curr_pt);

        self.curr_pt = self.arr2d.to_pt(idx + 1);

        self.arr2d.get_i(idx)
    }
}

impl<T> Index<IVec2> for Arr2D<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        if let Some(value) = self.get(index) {
            value
        } else {
            panic!("Attempted to access index {:?} in Arr2D of size {:?}", index, self.size);
        }
    }
}

impl<T> IndexMut<IVec2> for Arr2D<T> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        let size = self.size;
        if let Some(value) = self.get_mut(index) {
            value
        } else {
            panic!("Attempted to access index {:?} in Arr2D of size {:?}", index, size);
        }
    }
}

impl<T> Clone for Arr2D<T>
    where T: Clone
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size.clone(),
        }
    }
}