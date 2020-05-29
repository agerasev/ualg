use core::{
    ops::{
        Index, IndexMut,
        Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    mem::{MaybeUninit},
};
use num_traits::{Zero};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2<T> {
    data: [T; 2],
}

impl <T: Copy> Copy for Vec2<T> {}

impl<T> Vec2<T> {
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

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2<U> {
        let self_ = MaybeUninit::<Self>::new(self);
        let mut output = MaybeUninit::<Vec2<U>>::uninit();
        for i in 0..2 {
            unsafe {
                (&mut (*output.as_mut_ptr())[i] as *mut U).write(
                    f((&(*self_.as_ptr())[i] as *const T).read())
                );
            }
        }
        unsafe { output.assume_init() }
    }

    pub fn map_assign<R, F: Fn(&mut T, R)>(&mut self, f: F, other: Vec2<R>) {
        let other_ = MaybeUninit::<Vec2<R>>::new(other);
        for i in 0..2 {
            unsafe {
                f(&mut self[i], (&(*other_.as_ptr())[i] as *const R).read());
            }
        }
    }

    pub fn zip<R>(self, other: Vec2<R>) -> Vec2<(T, R)> {
        let self_ = MaybeUninit::<Self>::new(self);
        let other_ = MaybeUninit::<Vec2<R>>::new(other);
        let mut output = MaybeUninit::<Vec2<(T, R)>>::uninit();
        for i in 0..2 {
            unsafe {
                (&mut (*output.as_mut_ptr())[i] as *mut (T, R)).write((
                    (&(*self_.as_ptr())[i] as *const T).read(),
                    (&(*other_.as_ptr())[i] as *const R).read(),
                ));
            }
        }
        unsafe { output.assume_init() }
    }
}

impl<T: Copy> Vec2<T> {
    pub fn from_slice(data: &[T]) -> Self {
        Self { data: [data[0], data[1]] }
    }
    pub fn from_slice_ext(data: &[T], stride: usize) -> Self {
        Self { data: [data[0*stride], data[1*stride]] }
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}
impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl<R, T: Add<R>> Add<Vec2<R>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn add(self, other: Vec2<R>) -> Self::Output {
        self.zip(other).map(|(x, y)| x + y)
    }
}
impl<R, T: Sub<R>> Sub<Vec2<R>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Vec2<R>) -> Self::Output {
        self.zip(other).map(|(x, y)| x - y)
    }
}
impl<R, T: Mul<R>> Mul<Vec2<R>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Vec2<R>) -> Self::Output {
        self.zip(other).map(|(x, y)| x * y)
    }
}
impl<R, T: Div<R>> Div<Vec2<R>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn div(self, other: Vec2<R>) -> Self::Output {
        self.zip(other).map(|(x, y)| x / y)
    }
}
impl<R, T: Rem<R>> Rem<Vec2<R>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn rem(self, other: Vec2<R>) -> Self::Output {
        self.zip(other).map(|(x, y)| x % y)
    }
}

impl<R, T: AddAssign<R>> AddAssign<Vec2<R>> for Vec2<T> {
    fn add_assign(&mut self, other: Vec2<R>) {
        self.map_assign(|x, y| *x += y, other);
    }
}
impl<R, T: SubAssign<R>> SubAssign<Vec2<R>> for Vec2<T> {
    fn sub_assign(&mut self, other: Vec2<R>) {
        self.map_assign(|x, y| *x -= y, other);
    }
}
impl<R, T: MulAssign<R>> MulAssign<Vec2<R>> for Vec2<T> {
    fn mul_assign(&mut self, other: Vec2<R>) {
        self.map_assign(|x, y| *x *= y, other);
    }
}
impl<R, T: DivAssign<R>> DivAssign<Vec2<R>> for Vec2<T> {
    fn div_assign(&mut self, other: Vec2<R>) {
        self.map_assign(|x, y| *x /= y, other);
    }
}
impl<R, T: RemAssign<R>> RemAssign<Vec2<R>> for Vec2<T> {
    fn rem_assign(&mut self, other: Vec2<R>) {
        self.map_assign(|x, y| *x %= y, other);
    }
}

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2::new(T::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.x().is_zero() &&
        self.y().is_zero()
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

    #[test]
    fn arithmetics() {
        assert_eq!(Vec2::new(1, 2) + Vec2::new(3, 4), Vec2::new(4, 6));
        assert_eq!(Vec2::new(1, 4) - Vec2::new(3, 2), Vec2::new(-2, 2));
        assert_eq!(Vec2::new(1, 2) * Vec2::new(3, 4), Vec2::new(3, 8));
        assert_eq!(Vec2::new(3, 4) / Vec2::new(1, 2), Vec2::new(3, 2));
        assert_eq!(Vec2::new(5, 4) % Vec2::new(2, 3), Vec2::new(1, 1));
        
        let mut v = Vec2::new(1, 2);
        v += Vec2::new(3, 4);
        assert_eq!(v, Vec2::new(4, 6));

        let mut v = Vec2::new(1, 4);
        v -= Vec2::new(3, 2);
        assert_eq!(v, Vec2::new(-2, 2));

        let mut v = Vec2::new(1, 2);
        v *= Vec2::new(3, 4);
        assert_eq!(v, Vec2::new(3, 8));

        let mut v = Vec2::new(3, 4);
        v /= Vec2::new(1, 2);
        assert_eq!(v, Vec2::new(3, 2));

        let mut v = Vec2::new(5, 4);
        v %= Vec2::new(2, 3);
        assert_eq!(v, Vec2::new(1, 1));
    }
}
