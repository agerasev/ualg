use std::ops::{Index, IndexMut};
use num_traits::{self, Zero};


pub trait Num: num_traits::Num + Copy {}

pub trait StaticUsize {
    const VALUE: usize;
}
pub trait StaticArray<T: Num> : StaticUsize {
    type Array;
    fn filled(value: T) -> Self::Array;
}
pub trait Dim<T: Num> : StaticArray<T> {}

pub struct N0 {}
impl StaticUsize for N0 {
    const VALUE: usize = 0;
}
impl<T: Num> StaticArray<T> for N0 {
    type Array = [T; 0];
    fn filled(value: T) -> Self::Array {
        return [value; 0];
    }
}

pub struct N1 {}
impl StaticUsize for N1 {
    const VALUE: usize = 1;
}
impl<T: Num> StaticArray<T> for N1 {
    type Array = [T; 1];
    fn filled(value: T) -> Self::Array {
        return [value; 1];
    }
}

pub struct N2 {}
impl StaticUsize for N2 {
    const VALUE: usize = 2;
}
impl<T: Num> StaticArray<T> for N2 {
    type Array = [T; 2];
    fn filled(value: T) -> Self::Array {
        return [value; 2];
    }
}

pub struct N3 {}
impl StaticUsize for N3 {
    const VALUE: usize = 3;
}
impl<T: Num> StaticArray<T> for N3 {
    type Array = [T; 3];
    fn filled(value: T) -> Self::Array {
        return [value; 3];
    }
}

pub struct N4 {}
impl StaticUsize for N4 {
    const VALUE: usize = 4;
}
impl<T: Num> StaticArray<T> for N4 {
    type Array = [T; 4];
    fn filled(value: T) -> Self::Array {
        return [value; 4];
    }
}


#[allow(non_camel_case_types)]
pub struct vec<T: Num, N: Dim<T>> {
    v: <N as StaticArray<T>>::Array,
}

impl<T: Num, N: Dim<T>> vec<T, N> {
    pub fn new() -> Self {
        vec::<T, N> {
            v: <N as StaticArray<T>>::filled(T::zero()),
        }
    }
    //pub fn data(&self) -> 
}
/*
impl<T: Num, N: Dim<T>> Index<usize> for vec<T, N> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        self.data[i]
    }
}
*/