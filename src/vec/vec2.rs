use core::{
    ops::{
        Index, IndexMut,
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    mem::{MaybeUninit},
};
use num_traits::{Num, Zero};
use alga::{general as alg, linear as linalg};
use crate::Scal;


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
    pub fn map_update<F: Fn(&mut T)>(&mut self, f: F) {
        for i in 0..2 {
            f(&mut self[i]);
        }
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

impl<T: Clone> Vec2<T> {
    pub fn from_slice(data: &[T]) -> Self {
        Self::new(
            data[0].clone(),
            data[1].clone(),
        )
    }
    pub fn from_slice_ext(data: &[T], stride: usize) -> Self {
        Self::new(
            data[0*stride].clone(),
            data[1*stride].clone(),
        )
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

impl<T: Neg> Neg for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}
impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn add(self, other: Self) -> Self::Output {
        self.zip(other).map(|(x, y)| x + y)
    }
}
impl<T: Sub> Sub for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Self) -> Self::Output {
        self.zip(other).map(|(x, y)| x - y)
    }
}
impl<T: Mul> Mul for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Self) -> Self::Output {
        self.zip(other).map(|(x, y)| x * y)
    }
}
impl<T: Div> Div for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn div(self, other: Self) -> Self::Output {
        self.zip(other).map(|(x, y)| x / y)
    }
}
impl<T: Rem> Rem for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn rem(self, other: Self) -> Self::Output {
        self.zip(other).map(|(x, y)| x % y)
    }
}
impl<T: Clone + Mul> Mul<T> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn mul(self, other: T) -> Self::Output {
        self.map(|x| x * other.clone())
    }
}
impl<T: Clone + Div> Div<T> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn div(self, other: T) -> Self::Output {
        self.map(|x| x / other.clone())
    }
}
impl<T: Clone + Rem> Rem<T> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn rem(self, other: T) -> Self::Output {
        self.map(|x| x % other.clone())
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.map_assign(|x, y| *x += y, other);
    }
}
impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.map_assign(|x, y| *x -= y, other);
    }
}
impl<T: MulAssign> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, other: Self) {
        self.map_assign(|x, y| *x *= y, other);
    }
}
impl<T: DivAssign> DivAssign for Vec2<T> {
    fn div_assign(&mut self, other: Self) {
        self.map_assign(|x, y| *x /= y, other);
    }
}
impl<T: RemAssign> RemAssign for Vec2<T> {
    fn rem_assign(&mut self, other: Self) {
        self.map_assign(|x, y| *x %= y, other);
    }
}
impl<T: Clone + MulAssign> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, other: T) {
        self.map_update(|x| *x *= other.clone());
    }
}
impl<T: Clone + DivAssign> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, other: T) {
        self.map_update(|x| *x /= other.clone());
    }
}
impl<T: Clone + RemAssign> RemAssign<T> for Vec2<T> {
    fn rem_assign(&mut self, other: T) {
        self.map_update(|x| *x %= other.clone());
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

impl<T: Scal> alg::AbstractMagma<alg::Additive> for Vec2<T> {
    fn operate(&self, other: &Self) -> Self {
        self.clone() + other.clone()
    }
}
impl<T: Scal> alg::Identity<alg::Additive> for Vec2<T> {
    fn identity() -> Self {
        Self::zero()
    }
}
impl<T: Scal> alg::TwoSidedInverse<alg::Additive> for Vec2<T> {
    fn two_sided_inverse(&self) -> Self {
        -self.clone()
    }
}
impl<T: Scal> alg::AbstractSemigroup<alg::Additive> for Vec2<T> {}
impl<T: Scal> alg::AbstractQuasigroup<alg::Additive> for Vec2<T> {}
impl<T: Scal> alg::AbstractMonoid<alg::Additive> for Vec2<T> {}
impl<T: Scal> alg::AbstractLoop<alg::Additive> for Vec2<T> {}
impl<T: Scal> alg::AbstractGroup<alg::Additive> for Vec2<T> {}
impl<T: Scal> alg::AbstractGroupAbelian<alg::Additive> for Vec2<T> {}
impl<T: Scal + alg::AbstractRingCommutative> alg::AbstractModule for Vec2<T> {
    type AbstractRing = T;
    fn multiply_by(&self, r: Self::AbstractRing) -> Self {
        self.clone()*r.clone()
    }
}
impl<T: Scal + alg::RingCommutative> alg::Module for Vec2<T> {
    type Ring = T;
}
impl<T: Scal + alg::Field> linalg::VectorSpace for Vec2<T> {
    type Field = T;
}
// TODO: impl EuclideanSpace for Vec2

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
        assert_eq!(-Vec2::new(1, 2), Vec2::new(-1, -2));

        assert_eq!(Vec2::new(1, 2) + Vec2::new(3, 4), Vec2::new(4, 6));
        assert_eq!(Vec2::new(1, 4) - Vec2::new(3, 2), Vec2::new(-2, 2));
        assert_eq!(Vec2::new(1, 2) * Vec2::new(3, 4), Vec2::new(3, 8));
        assert_eq!(Vec2::new(3, 4) / Vec2::new(1, 2), Vec2::new(3, 2));
        assert_eq!(Vec2::new(5, 4) % Vec2::new(2, 3), Vec2::new(1, 1));
        assert_eq!(Vec2::new(1, 3)*2, Vec2::new(2, 6));
        assert_eq!(Vec2::new(1, 3)/2, Vec2::new(0, 1));
        assert_eq!(Vec2::new(5, 6)%3, Vec2::new(2, 0));
        
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

        let mut v = Vec2::new(1, 3);
        v *= 2;
        assert_eq!(v, Vec2::new(2, 6));

        let mut v = Vec2::new(1, 3);
        v /= 2;
        assert_eq!(v, Vec2::new(0, 1));

        let mut v = Vec2::new(5, 6);
        v %= 3;
        assert_eq!(v, Vec2::new(2, 0));
    }
}
