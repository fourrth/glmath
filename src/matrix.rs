use crate::vector::*;
use crate::Element;

use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

macro_rules! GENERATE_MATRIX {
    ($($n:expr),*) => {
        $(
            // Generate the struct using paste
            paste::item! {

                #[repr(C)]
                #[derive(Debug, Clone, Copy)]
                pub struct [<Matrix $n x $n>]<T:Element>(pub [[<Vector $n>]<T>; $n]);

                impl<T: Element> From<[[<Vector $n>]<T>; $n]> for [<Matrix $n x $n>]<T> {
                    #[inline(always)]
                    fn from(value: [[<Vector $n>]<T>; $n]) -> Self {
                        Self(value)
                    }
                }

                impl<T: Element> From<[T; $n * $n]> for [<Matrix $n x $n>]<T> {
                    #[inline(always)]
                    fn from(value: [T; $n * $n]) -> Self {
                        [<Matrix $n x $n>](
                            unsafe { *(value.as_ptr() as *const [[T; $n]; $n]) }.map(move |arr| [<Vector $n>]::from(arr)),
                        )
                    }
                }

                impl<T: Element> From<&[T; $n * $n]> for [<Matrix $n x $n>]<T> {
                    #[inline(always)]
                    fn from(value: &[T; $n * $n]) -> Self {
                        Self::from(value.clone())
                    }
                }

                impl<T: Element> Index<usize> for [<Matrix $n x $n>]<T> {
                    type Output = [<Vector $n>]<T>;
                    #[inline(always)]
                    fn index(&self, index: usize) -> &Self::Output {
                        self.0.index(index)
                    }
                }

                impl<T: Element> IndexMut<usize> for [<Matrix $n x $n>]<T> {
                    #[inline(always)]
                    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                        self.0.index_mut(index)
                    }
                }

                impl<T: Element> IntoIterator for [<Matrix $n x $n>]<T> {
                    type Item = [<Vector $n>]<T>;
                    type IntoIter = std::array::IntoIter<[<Vector $n>]<T>, $n>;
                    #[inline(always)]
                    fn into_iter(self) -> Self::IntoIter {
                        self.0.into_iter()
                    }
                }

                impl<T: Element> PartialEq for [<Matrix $n x $n>]<T> {
                    fn eq(&self, other: &Self) -> bool {
                        for (v1, v2) in self.into_iter().zip(other.into_iter()) {
                            if v1 != v2 {
                                return false;
                            }
                        }
                        true
                    }
                }

                impl<T: Element> [<Matrix $n x $n>]<T> {

                    /// Creates a new Matrix using uninitialized data
                    #[inline(always)]
                    pub unsafe fn new_uninit() -> Self {
                         Self(unsafe {MaybeUninit::<[[<Vector $n>]<T>; $n]>::uninit().assume_init()})
                    }

                    /// Does element-wise addition
                    #[inline(always)]
                    pub fn add(mut self, addend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx].add(addend[cx]);
                        }
                        self
                    }

                    /// Does element-wise subtraction
                    #[inline(always)]
                    pub fn sub(mut self, subtrahend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx].sub(subtrahend[cx]);
                        }
                        self
                    }

                    /// Does element-wise division by a scalar
                    #[inline(always)]
                    pub fn div_scalar(self, scalar: T) -> Self {
                        Self(self.0.map(|ca| ca.div_scalar(scalar)))
                    }

                    /// Does element-wise multiplication by a scalar
                    #[inline(always)]
                    pub fn mul_scalar(self, scalar: T) -> Self {
                        Self(self.0.map(|ca| ca.mul_scalar(scalar)))
                    }

                    /// Gives the transpose of the Matrix
                    #[inline(always)]
                    pub fn transpose(self) -> Self {
                        let mut ret = unsafe {Self::new_uninit()};
                        for cx in 0..$n {
                            for cy in 0..$n {
                                ret[cy][cx] = self[cx][cy];
                            }
                        }
                        ret
                    }

                    /// Gives the trace of the Matrix
                    #[inline(always)]
                    pub fn trace(self) -> T {
                        let mut sum = T::zero();
                        for cx in 0..$n {
                            sum = sum + self[cx][cx]
                        }
                        sum
                    }

                    /// Multiplies two Matrix's together
                    #[inline(always)]
                    pub fn mul_matrix(self, other: Self) -> Self {
                        let other_transpose = other.transpose();
                        let mut data = unsafe { MaybeUninit::<[T; $n *$n]>::uninit().assume_init() };

                        for cx in 0..$n {
                            for cy in 0..$n {
                                data[cx * $n + cy] = self[cx].mul_inner(other_transpose[cy]);
                            }
                        }

                        Self::from(data)
                    }

                    /// Does mul_matrix but power times
                    /// Raises the matrix to some power
                    #[inline(always)]
                    pub fn powi(self,power:usize) -> Self {
                        use std::hint::black_box;
                        if power == 0 {
                            return Self::ident();
                        }
                        let mut ret = self;
                        for _ in 0..power-1 {
                            ret = black_box(self.mul_matrix(ret));
                            black_box(ret);
                        }
                        black_box(ret)
                    }

                    /// Does the same as `Self::inverse()`,
                    /// but does not check if `self.det() = 0`.
                    /// Use only if you know that it is impossible
                    /// the the determinant for your data cannot
                    /// be 0 (like in homogenous 3D, Matrix4x4)
                    #[inline(always)]
                    pub fn inverse_unchecked(self) -> Self {
                        self.inverse_inner(self.det())
                    }
                }// impl end
            }
        )*
    };
}

impl<T: Element> Matrix2x2<T> {
    #[inline(always)]
    // Gives the Identity Matrix
    pub fn ident() -> Self {
        Self::from([T::one(), T::zero(), T::zero(), T::one()])
    }

    /// Gives the determinant of the Matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::matrix::Matrix2x2;
    /// use approx::assert_relative_eq;
    ///
    ///  let m1 = Matrix2x2::from([1f32, 2f32, 3f32, 4f32]);
    ///
    /// assert_relative_eq!(m1.det(), -2f32);
    /// ```
    #[inline(always)]
    pub fn det(self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    #[inline(always)]
    fn inverse_inner(self, det: T) -> Self {
        Self::from([self[1][1], -self[0][1], -self[1][0], self[0][0]]).div_scalar(det)
    }

    /// Inverts the Matrix, but first checks
    /// to see if `self.det() == 0`
    #[inline(always)]
    pub fn inverse(self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() {
            None
        } else {
            Some(self.inverse_inner(det))
        }
    }
}

impl<T: Element> Matrix3x3<T> {
    /// Gives the Identity Matrix
    #[inline(always)]
    pub fn ident() -> Self {
        Self::from([
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ])
    }
    /// Gives the determinant of the Matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::matrix::Matrix3x3;
    /// use approx::assert_relative_eq;
    ///
    ///  let m1 = Matrix3x3::from([-1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32]);
    ///
    /// assert_relative_eq!(m1.det(), 6f32);
    /// ```
    #[inline(always)]
    pub fn det(self) -> T {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }
    #[inline(always)]
    fn inverse_inner(self, det: T) -> Self {
        Matrix3x3::from([
            self[1][1] * self[2][2] - self[1][2] * self[2][1],
            -(self[0][1] * self[2][2] - self[0][2] * self[2][1]),
            self[0][1] * self[1][2] - self[0][2] * self[1][1],
            -(self[1][0] * self[2][2] - self[1][2] * self[2][0]),
            self[0][0] * self[2][2] - self[0][2] * self[0][2] * self[2][0],
            -(self[0][0] * self[1][2] - self[0][2] * self[1][0]),
            self[1][0] * self[2][1] - self[1][1] * self[2][0],
            -(self[0][0] * self[2][1] - self[0][1] * self[2][0]),
            (self[0][0] * self[1][1] - self[0][1] * self[1][0]),
        ])
        .div_scalar(det)
    }

    /// Inverts the Matrix, but first checks
    /// to see if `self.det() == 0`
    #[inline(always)]
    pub fn inverse(self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() {
            None
        } else {
            Some(self.inverse_inner(det))
        }
    }
}

impl<T: Element> Matrix4x4<T> {
    /// Gives the Identity Matrix
    #[inline(always)]
    pub fn ident() -> Self {
        Self::from([
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ])
    }

    /// Gives the determinant of the Matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::matrix::Matrix4x4;
    /// use approx::assert_relative_eq;
    ///
    ///  let m1 = Matrix4x4::from([1f32, 3f32, 5f32, 9f32, 1f32, 3f32, 1f32, 7f32, 4f32, 3f32, 9f32, 7f32, 5f32, 2f32, 0f32, 9f32]);
    ///
    /// assert_relative_eq!(m1.det(), -376f32);
    /// ```
    #[inline(always)]
    pub fn det(self) -> T {
        self[0][0]
            * (self[1][1] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
                - self[1][2] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
                + self[1][3] * (self[2][1] * self[3][2] - self[2][2] * self[3][1]))
            - self[0][1]
                * (self[1][0] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
                    - self[1][2] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
                    + self[1][3] * (self[2][0] * self[3][2] - self[2][2] * self[3][0]))
            + self[0][2]
                * (self[1][0] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
                    - self[1][1] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
                    + self[1][3] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]))
            - self[0][3]
                * (self[1][0] * (self[2][1] * self[3][2] - self[2][2] * self[3][1])
                    - self[1][1] * (self[2][0] * self[3][2] - self[2][2] * self[3][0])
                    + self[1][2] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]))
    }

    #[inline(always)]
    fn inverse_inner(self, det: T) -> Self {
        // using the Carley-Hamilton
        let tr = self.trace();

        let self_pow2 = self.mul_matrix(self);
        let tr_2 = self_pow2.trace();

        let self_pow3 = self_pow2.mul_matrix(self);
        let two = T::one() + T::one();
        let three = two + T::one();
        let six = two * three;

        Self::ident()
            .mul_scalar(((tr.powi(3)) - (three * tr * tr_2) + (two * self_pow3.trace())) / six)
            .sub(self.mul_scalar((T::one() / two) * ((tr.powi(2)) - (tr_2))))
            .add(self_pow2.mul_scalar(tr))
            .sub(self_pow3)
            .div_scalar(det)
    }

    /// Inverts the Matrix, but first checks
    /// to see if `self.det() == 0`
    #[inline(always)]
    pub fn inverse(self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() {
            None
        } else {
            Some(self.inverse_inner(det))
        }
    }
}

GENERATE_MATRIX!(2, 3, 4);
