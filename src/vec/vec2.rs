use core::ops::{
    Index, IndexMut,
    Add, Sub, Mul, Div,
    AddAssign, SubAssign, MulAssign, DivAssign,
};
use num_traits::{Zero};
use crate::Scal;


pub struct Vec2<T: Scal> {
    data: [T; 2],
}

impl<T: Scal> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { data: [x, y] }
    }

    pub fn as_array(&self) -> &[T; 2] {
        &self.data
    }
    pub fn as_array_mut(&mut self) -> &mut [T; 2] {
        &mut self.data
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    
    pub fn x(&self) -> &T {
        unsafe { self.data.get_unchecked(0) }
    }
    pub fn y(&self) -> &T {
        unsafe { self.data.get_unchecked(1) }
    }
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.data.get_unchecked_mut(0) }
    }
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.data.get_unchecked_mut(1) }
    }
}

impl<T: Scal + Copy> Vec2<T> {
    pub fn from_slice(data: &[T]) -> Self {
        Self { data: [data[0], data[1]] }
    }
    pub fn from_slice_ext(data: &[T], stride: usize) -> Self {
        Self { data: [data[0], data[1*stride]] }
    }
}

impl<T: Scal> Index<usize> for Vec2<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}
impl<T: Scal> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access() {
        let v = Vec2::<i32>::new(1, 2);
        assert_eq!(*v.x(), 1);
        assert_eq!(*v.y(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
    }
}
