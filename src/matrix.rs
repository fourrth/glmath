use crate::vector::*;
use crate::Element;
use core::ops::{Index, IndexMut};

macro_rules! GENERATE_MATRIX {
    ($($n:expr),*) => {
        $(
            // Generate the struct using paste
            paste::item! {

                #[repr(C)]
                #[derive(Debug, Clone, Copy)]
                pub struct [<Matrix $n x $n>]<T:Element>(pub [[<Vector $n>]<T>; $n]);

                impl<T: Element> From<[[<Vector $n>]<T>; $n]> for [<Matrix $n x $n>]<T> {
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
                    #[inline(always)]
                    pub fn add(mut self, addend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx].add(addend[cx]);
                        }
                        self
                    }

                    #[inline(always)]
                    pub fn sub(mut self, subtrahend: Self) -> Self {
                        for cx in 0..$n {
                            self[cx] = self[cx].sub(subtrahend[cx]);
                        }
                        self
                    }

                    #[inline(always)]
                    pub fn div_scalar(self, scalar: T) -> Self {
                        Self(self.0.map(|ca| ca.div_scalar(scalar)))
                    }

                    #[inline(always)]
                    pub fn mul_scalar(self, scalar: T) -> Self {
                        Self(self.0.map(|ca| ca.mul_scalar(scalar)))
                    }
                }// impl end
            }
        )*
    };
}

GENERATE_MATRIX!(2, 3, 4);
