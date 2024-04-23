use crate::Element;

/// Gives linear interpolation between
/// two vectors via parameter t
///
/// # Examples
///
/// ```
/// use glmath::scalar::lerp;
/// use approx::assert_relative_eq;
///
/// let v0 = 0f32;
/// let v1 = 4f32;
/// let v2 = 12f32;
///
/// assert_relative_eq!(lerp(v0, v1, 0f32), 0f32);
/// assert_relative_eq!(lerp(v0, v2, 0f32), 0f32);
///
/// assert_relative_eq!(lerp(v1, v2, 0f32), 4f32);
/// assert_relative_eq!(lerp(v1, v2, 0.25f32), 6f32);
/// assert_relative_eq!(lerp(v1, v2, 0.5f32), 8f32);
/// assert_relative_eq!(lerp(v1, v2, 0.75f32), 10f32);
/// assert_relative_eq!(lerp(v1, v2, 1f32), 12f32);
///
/// assert_relative_eq!(lerp(v1, v2, 2f32), 20f32);
/// ```
#[inline(always)]
pub fn lerp<T: Element>(value0: T, value1: T, t: T) -> T {
    value0 + (value1 - value0) * t
}
