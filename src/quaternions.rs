use std::ops::{Index, IndexMut};

//TODO: Examples/Test for everything
use crate::{
    vector::{Vector3, Vector4},
    Element,
};

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
impl<T: Element> From<(Vector3<T>, T)> for Quaternion<T> {
    fn from(value: (Vector3<T>, T)) -> Self {
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
    /// Separates the Quaternion into Vector and Scalar components.
    /// Note that we define Quaternions to be (x,y,z,w)
    #[inline(always)]
    pub fn seperate(self) -> (Vector3<T>, T) {
        (Vector3([self[0], self[1], self[2]]), self[3])
    }
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
    /// Multiply two Quaternions
    #[inline(always)]
    pub fn mul(self, other: Self) -> Self {
        let (self_v3, self_w) = self.seperate();
        let (other_v3, other_w) = other.seperate();
        Quaternion::from((
            self_v3
                .mul_cross(other_v3)
                .add(self_v3.mul_scalar(other_w).add(other_v3.mul_scalar(self_w))),
            self_w * other_w - self_v3.mul_inner(other_v3),
        ))
    }
    /// Conjugate the Quaternion
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self::from([-self[0], -self[1], -self[2], self[3]])
    }
}
