use crate::scalar::lerp;

use super::Element;

#[cfg(feature = "random_vectors")]
use once_cell::unsync::Lazy;

use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
#[cfg(feature = "random_vectors")]
static mut RNG_GEN: Lazy<rand::rngs::ThreadRng> = Lazy::new(|| rand::rngs::ThreadRng::default());

macro_rules! GENERATE_VEC {
    ($($n:expr),*) => {
        $(
            // Generate the struct using paste
            paste::item! {

                /// Generates a Vector with random elements.
                /// Since T is a float, it will generate a value [0,1)
                #[cfg(feature = "random_vectors")]
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

                #[cfg(feature = "bytemuck")]
                unsafe impl<T: Element> bytemuck::Pod for [<Vector $n>]<T> {}

                #[cfg(feature = "bytemuck")]
                unsafe impl<T: Element> bytemuck::Zeroable for [<Vector $n>]<T> {
                    fn zeroed() -> Self {
                        Self([T::zero();$n])
                    }
                }


                #[repr(transparent)]
                #[derive(Debug, Clone, Copy)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
                        self.eq_fast(*other,T::epsilon())
                    }
                }

                impl<T: Element> [<Vector $n>]<T> {
                    #[inline(always)]
                    pub fn new_uninit() -> Self {
                         Self(unsafe {MaybeUninit::<[T;$n]>::uninit().assume_init()})
                    }

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
                        self.div_scalar(self.len() + T::epsilon())
                    }

                    /// Gets the distance between both Vectors
                    #[inline(always)]
                    pub fn dist(self, other: Self) -> T {
                        other.sub(self).len()
                    }

                    /// Gets the angle between two Vectors
                    #[inline(always)]
                    pub fn angle(self, other: Self) -> T {
                        let a = self.mul_inner(other);
                        let b = self.len() * other.len();
                        num::clamp(a/b, -T::one(), T::one()).acos()
                    }

                    /// Does [`crate::scalar::lerp`] but on each element
                    /// of the Vector. Result is the linear interpolation
                    /// between the two Vectors
                    #[inline(always)]
                    pub fn lerp(mut self, other: Self, t: T) -> Self {
                        for cx in 0..$n {
                            self[cx] = lerp(self[cx], other[cx], t);
                        }
                        self
                    }

                    /// Does by-value comparison to see if two Vector's are equal
                    /// by using the given epsilon value
                    #[inline(always)]
                    pub fn eq_fast(self, other: Self, epsilon:T) -> bool {

                        for (ca,cb) in self.into_iter().zip(other.into_iter()) {
                            if (ca-cb).abs() > epsilon {
                                return false;
                            }
                        }
                        true
                    }
                }// impl end
            }
        )*
    };
}

// Probably should convert this all into a proc macro eventually
impl<T: Element> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self(value.into())
    }
}
impl<T: Element> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self(value.into())
    }
}
impl<T: Element> From<(T, T, T, T)> for Vector4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self(value.into())
    }
}

impl<T: Element> From<(Vector3<T>, T)> for Vector4<T> {
    fn from(value: (Vector3<T>, T)) -> Self {
        Vector4::from(unsafe { *(&value as *const _ as *const [T; 4]) })
    }
}

impl<T: Element> From<(Vector2<T>, Vector2<T>)> for Vector4<T> {
    fn from(value: (Vector2<T>, Vector2<T>)) -> Self {
        Vector4::from(unsafe { *(&value as *const _ as *const [T; 4]) })
    }
}

impl<T: Element> From<(Vector2<T>, T)> for Vector3<T> {
    fn from(value: (Vector2<T>, T)) -> Self {
        Vector3::from(unsafe { *(&value as *const _ as *const [T; 3]) })
    }
}

impl<T: Element> Vector3<T> {
    #[inline(always)]
    /// Creates a Vector from spherical coordinates
    /// theta is angle from top axis downwards
    /// phi is the standard polar angle
    /// magnitude is the scaling magnitude
    //TODO: add more/examples to documentation
    pub fn from_spherical(theta: T, phi: T, magnitude: T) -> Self {
        let (theta_sin, theta_cos) = theta.sin_cos();
        let (phi_sin, phi_cos) = phi.sin_cos();

        Self([theta_sin * phi_cos, theta_sin * phi_sin, theta_cos]).mul_scalar(magnitude)
    }
    /// Does cross product
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::vector::Vector3;
    /// let i_hat = Vector3::from([1f32, 0f32, 0f32]);
    /// let j_hat = Vector3::from([0f32, 1f32, 0f32]);
    /// let k_hat = Vector3::from([0f32, 0f32, 1f32]);
    ///
    /// let origin: Vector3<f32> = Vector3::from([0f32, 0f32, 0f32]);
    ///
    /// assert_eq!(i_hat.mul_cross(j_hat), k_hat);
    /// assert_eq!(k_hat.mul_cross(i_hat), j_hat);
    /// assert_eq!(j_hat.mul_cross(k_hat), i_hat);
    ///
    /// assert_eq!(k_hat.mul_cross(j_hat), i_hat.mul_scalar(-1f32));
    ///
    /// assert_eq!(i_hat.mul_cross(i_hat), origin);
    /// ```
    #[inline(always)]
    pub fn mul_cross(self, crossed: Self) -> Self {
        Self([
            self[1] * crossed[2] - self[2] * crossed[1],
            self[2] * crossed[0] - self[0] * crossed[2],
            self[0] * crossed[1] - self[1] * crossed[0],
        ])
    }
}

impl<T: Element> Vector2<T> {
    #[inline(always)]
    /// Creates a Vector from polar coordinates
    /// phi is the standard polar angle
    /// magnitude is the scaling magnitude
    //TODO: add more/examples to documentation
    pub fn from_polar(phi: T, magnitude: T) -> Self {
        let (phi_sin, phi_cos) = phi.sin_cos();

        Self([phi_cos, phi_sin]).mul_scalar(magnitude)
    }
    /// Gets the perpendicular Vector
    /// See example for difference to [`crate::vector::Vector2::perp2`]
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::vector::Vector2;
    /// let v1 = Vector2::from([1f32,2f32]);
    /// let p2 = v1.perp1();
    ///
    /// assert_eq!(p2, Vector2::from([-2f32,1f32]));
    /// assert_eq!(v1.mul_inner(p2), 0f32);
    /// ```
    #[inline(always)]
    pub fn perp1(self) -> Self {
        Self([-self[1], self[0]])
    }
    /// Gets the perpendicular Vector
    /// See example for difference to [`crate::vector::Vector2::perp1`]
    ///
    /// # Examples
    ///
    /// ```
    /// use glmath::vector::Vector2;
    /// let v1 = Vector2::from([1f32,2f32]);
    /// let p2 = v1.perp2();
    ///
    /// assert_eq!(p2, Vector2::from([2f32,-1f32]));
    /// assert_eq!(v1.mul_inner(p2), 0f32);
    /// ```
    #[inline(always)]
    pub fn perp2(self) -> Self {
        Self([self[1], -self[0]])
    }
}

GENERATE_VEC!(2, 3, 4);
