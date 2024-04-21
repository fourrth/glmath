use super::Element;
use core::ops::{Index, IndexMut};

#[cfg(feature = "rand")]
use once_cell::unsync::Lazy;
#[cfg(feature = "rand")]
static mut RNG_GEN: Lazy<rand::rngs::ThreadRng> = Lazy::new(|| rand::rngs::ThreadRng::default());

macro_rules! GENERATE_VEC {
    ($($n:expr),*) => {
        $(
            // Generate the struct using paste
            paste::item! {

                /// Generates a vector with random elements.
                /// Since T is a float, it will generate a value [0,1)
                #[cfg(feature = "rand")]
                #[inline(always)]
                pub fn [<generate_rand_vector $n>]<T>() -> [<Vector $n>]<T>
                where
                    T: Element,
                    rand::distributions::Standard: rand::distributions::Distribution<T>,
                {
                    use std::mem::MaybeUninit;

                    use rand::Rng;

                    [<Vector $n>]::from(
                        unsafe { MaybeUninit::<[T; $n]>::uninit().assume_init() }
                            .map(|_| unsafe { RNG_GEN.gen::<T>() }),
                    )
                }

                #[derive(Debug, Clone, Copy)]
                pub struct [<Vector $n>]<T:Element>(pub [T; $n]);

                impl<T:Element> From<[T;$n]> for [<Vector $n>]<T> {
                    #[inline(always)]
                    fn from(value: [T;$n]) -> Self {
                        Self(value)
                    }
                }

                impl<T:Element> From<&[T;$n]> for [<Vector $n>]<T> {
                    #[inline(always)]
                    fn from(value: &[T;$n]) -> Self {
                        Self(value.clone())
                    }
                }

                impl<T:Element> Index<usize> for [<Vector $n>]<T> {
                    type Output = T;
                    #[inline(always)]
                    fn index(&self, index: usize) -> &Self::Output {
                        self.0.index(index)
                    }
                }

                impl<T:Element> IndexMut<usize> for [<Vector $n>]<T> {
                    #[inline(always)]
                    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                        self.0.index_mut(index)
                    }
                }

                impl<T: Element> IntoIterator for [<Vector $n>]<T> {
                    type Item = T;
                    type IntoIter = core::array::IntoIter<T, $n>;
                    #[inline(always)]
                    fn into_iter(self) -> Self::IntoIter {
                        self.0.into_iter()
                    }
                }

                impl<T: Element> PartialEq for [<Vector $n>]<T> {
                    fn eq(&self, other: &Self) -> bool {
                        for (ca, cb) in self.into_iter().zip(other.into_iter()) {
                            if (ca - cb).abs() > T::epsilon() // faster to just check one but idc
                            || (cb - ca).abs() > T::epsilon()
                            {
                                return false;
                            }
                        }
                        true
                    }
                }

                impl<T: Element> [<Vector $n>]<T> {
                    /// Does element-wise addition
                    #[inline(always)]
                    pub fn add(mut self, addend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx] + addend[cx];
                        }
                        self
                    }

                    /// Does element-wise subtraction
                    #[inline(always)]
                    pub fn sub(mut self, subtrahend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx] - subtrahend[cx];
                        }
                        self
                    }

                    /// Does scalar-wise division
                    #[inline(always)]
                    pub fn div_scalar(mut self, scalar: T) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx] / scalar;
                        }
                        self
                    }

                    /// Does scalar-wise multiplication
                    #[inline(always)]
                    pub fn mul_scalar(mut self, scalar: T) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx] * scalar;
                        }
                        self
                    }

                    /// Does inner product (aka dot product)
                    #[inline(always)]
                    pub fn mul_inner(self, other: Self) -> T {
                        self.into_iter()
                        .zip(other.into_iter())
                        .map(|(ca, cb)| ca * cb)
                        .sum()
                    }

                    /// Gets the length of the Vector
                    #[inline(always)]
                    pub fn len(self) -> T {
                        self.mul_inner(self).sqrt()
                    }

                    /// Gets the norm of the Vector
                    #[inline(always)]
                    pub fn norm(self) -> Self {
                        self.div_scalar(self.len())
                    }

                    /// Gets the distance between both Vectors
                    #[inline(always)]
                    pub fn dist(self, other: Self) -> T {
                        other.sub(self).len()
                    }

                    /// Gets the angle between two vectors
                    #[inline(always)]
                    pub fn angle(self, other: Self) -> T {
                        let a = self.mul_inner(other);
                        let b = self.len() * other.len();
                        num::clamp(a/b, -T::one(), T::one()).acos()
                    }
                }// impl end
            }
        )*
    };
}

impl<T: Element> Vector3<T> {
    /// Does cross product
    #[inline(always)]
    pub fn mul_cross(self, crossed: Self) -> Self {
        Self([
            self[1] * crossed[2] - self[2] * crossed[1],
            self[2] * crossed[0] - self[0] * crossed[2],
            self[0] * crossed[1] - self[1] * crossed[0],
        ])
    }
}

GENERATE_VEC!(2, 3, 4);
