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
    use super::*;
    #[test]
    fn test_vectors_create() {
        let data_2f = [1.0, 2.0];
        let data_3f = [1.0, 2.0, 3.0];
        let data_4f = [1f32, 2f32, 3f32, 4f32];

        let _vec2f = Vector2::from(data_2f);
        let _vec3f = Vector3::from(data_3f);
        let _vec4f = Vector4::from(data_4f);
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
}
