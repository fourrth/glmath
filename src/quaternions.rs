use std::ops::{Index, IndexMut};

//TODO: Examples/Test for everything
use crate::{vector::Vector4, Element};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T: Element>(Vector4<T>);

impl<T: Element> From<Vector4<T>> for Quaternion<T> {
    fn from(value: Vector4<T>) -> Self {
        Self(value)
    }
}
impl<T: Element> From<[T; 4]> for Quaternion<T> {
    fn from(value: [T; 4]) -> Self {
        Self(Vector4::from(value))
    }
}

impl<T: Element> Index<usize> for Quaternion<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T: Element> IndexMut<usize> for Quaternion<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T: Element> Quaternion<T> {
    /// Does element-wise addition
    #[inline(always)]
    pub fn add(self, addend: Self) -> Self {
        Self(self.0.add(addend.0))
    }
    /// Does element-wise subtraction
    #[inline(always)]
    pub fn sub(self, subtrahend: Self) -> Self {
        Self(self.0.sub(subtrahend.0))
    }
    /// Gets the norm of the Quaternion
    #[inline(always)]
    pub fn norm(self) -> Self {
        Self(self.0.norm())
    }
    /// Does scalar-wise division
    #[inline(always)]
    pub fn div_scalar(self, scalar: T) -> Self {
        Self(self.0.div_scalar(scalar))
    }
    /// Does scalar-wise multiplication
    #[inline(always)]
    pub fn mul_scalar(self, scalar: T) -> Self {
        Self(self.0.mul_scalar(scalar))
    }
    /// Does inner product (aka dot product)
    #[inline(always)]
    pub fn mul_inner(self, other: Self) -> T {
        self.0.mul_inner(other.0)
    }
    /// Gives the identity Quaternion
    #[inline(always)]
    pub fn identity() -> Self {
        Quaternion::from([T::zero(), T::zero(), T::zero(), T::one()])
    }
}
