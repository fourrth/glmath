use crate::Element;

/// Gives linear interpolation between
/// two values via parameter t
#[inline(always)]
pub fn lerp<T: Element>(value0: T, value1: T, t: T) -> T {
    value0 + (value1 - value0) * t
}
