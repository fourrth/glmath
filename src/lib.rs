use core::ops::{Index, IndexMut};

pub trait Element: core::fmt::Debug + std::iter::Sum + num::Float + Clone + Copy {}
impl<T: core::fmt::Debug + num::Float + std::iter::Sum + Clone + Copy> Element for T {}

macro_rules! GENERATE_VEC {
    ($($n:expr),*) => {
        $(
            // Generate the struct using paste
            paste::item! {
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

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use approx::assert_relative_eq;

    use super::*;
    #[test]
    fn test_vectors_basic() {
        let data_2f = [1.0, 2.0];
        let data_3f = [1.0, 2.0, 3.0];
        let data_4f = [1f32, 2f32, 3f32, 4f32];

        let _vec2f = Vector2::from(data_2f);
        let _vec3f = Vector3::from(data_3f);
        let _vec4f = Vector4::from(data_4f);

        assert_eq!(_vec2f, _vec2f);
    }

    #[test]
    fn test_vectors_add_sub() {
        let v1 = Vector4::from([4f32, 1f32, -5f32, 17f32]);
        let v2 = Vector4::from([-99f32, -0.05f32, 12f32, -17f32]);
        let ans = Vector4::from([-95f32, 0.95f32, 7f32, 0f32]);

        assert_eq!(v1.add(v2), ans);
        assert_eq!(ans.sub(v1), v2);
        assert_eq!(ans.sub(v2), v1);
    }

    #[test]
    fn test_vectors_mul_div_scalar() {
        let scalar = 4f32;
        let v1 = Vector4::from([0f32, 1f32, 3f32, 12f32]);
        let ans_mul = v1.mul_scalar(scalar);
        let ans_div = v1.div_scalar(scalar);

        assert_eq!(ans_mul, Vector4::from([0f32, 4f32, 12f32, 48f32]));
        assert_eq!(ans_div, Vector4::from([0f32, 0.25f32, 0.75f32, 3f32]));

        assert_eq!(v1, ans_mul.div_scalar(scalar));
        assert_eq!(v1, ans_div.mul_scalar(scalar));
        assert_eq!(ans_mul, v1.add(v1).add(v1).add(v1));
    }
    #[test]
    fn test_vectors_len_norm() {
        let v1 = Vector2::from([3f32, 4f32]);
        let v2 = Vector2::from([11f32, 60f32]);

        assert_relative_eq!(v1.len(), 5f32);
        assert_relative_eq!(v2.len(), 61f32);

        assert_eq!(v1.norm(), Vector2::from([0.6f32, 0.8f32]));
        assert_eq!(v2.norm(), Vector2::from([11f32 / 61f32, 60f32 / 61f32]));
    }

    #[test]
    fn test_vectors_mul_inner() {
        let theta = PI / 4f32;
        let (y, x) = theta.sin_cos();
        let v1 = Vector2::from([x, y]);
        let v1_perp1 = Vector2::from([-y, x]);
        let v1_perp2 = Vector2::from([y, -x]);

        assert_relative_eq!(v1.mul_inner(v1_perp1), 0f32);
        assert_relative_eq!(v1.mul_inner(v1_perp2), 0f32);

        assert_relative_eq!(v1_perp1.norm().mul_inner(v1_perp2.norm()), -1f32);
        assert_relative_eq!(v1.norm().mul_inner(v1.mul_scalar(5f32).norm()), 1f32);
    }

}
