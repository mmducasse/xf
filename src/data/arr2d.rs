use std::ops::{Index, IndexMut};

use crate::num::{ivec2::{IVec2, i2}, irect::IRect};


pub struct Arr2D<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Arr2D<T> {
    pub fn new(data: Vec<T>, width: usize) -> Self {
        assert!(width > 0);
        Self { data, width }
    }

    pub fn get(&self, pos: IVec2) -> Option<&T> {
        let i = self.to_idx(pos);
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

    pub fn set(&mut self, pt: IVec2, value: T) -> bool {
        if self.bounds().contains(pt) {
            let idx = self.to_idx(pt);
            self.data[idx] = value;
            true
        } else { false }
    }

    pub fn set_i(&mut self, idx: usize, value: T) -> bool {
        if idx < self.data.len() {
            self.data[idx] = value;
            true
        } else { false }
    }

    pub fn size(&self) -> IVec2 {
        let last_idx = self.count().max(1) - 1;
        let height = self.to_pt(last_idx).y + 1;

        i2(self.width as i32, height as i32)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn bounds(&self) -> IRect {
        IRect::of_size(self.size())
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
        let x = (i % self.width) as i32;
        let y = (i / self.width) as i32;
        i2(x, y)
    }

    pub fn to_idx(&self, pt: IVec2) -> usize {
        let x = pt.x as usize;
        let y = pt.y as usize;
        y * self.width + x
    }

    pub fn iter(&self) -> Arr2DIter<'_, T> {
        Arr2DIter { arr2d: self, curr_pt: i2(0, 0) }
    }
}

impl<T: Clone> Arr2D<T> {
    pub fn default(default_value: T, size: IVec2) -> Self {
        let data = vec![default_value; size.product() as usize];
        Self::new(data, size.x as usize)
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

    pub fn copy_from(&mut self, other: &Arr2D<T>, src: IRect, dst: IVec2) {
        for src_pt in src.iter() {
            let offset = src_pt - src.pos;
            let dst_pt = dst + offset;

            if let Some(color) = other.get(src_pt) {
                self.set(dst_pt, *color);
            }
        }
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
            panic!("Attempted to access index {:?} in Arr2D of size {:?}", index, self.size());
        }
    }
}

impl<T> IndexMut<IVec2> for Arr2D<T> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        let size = self.size();
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
            width: self.width,
        }
    }
}