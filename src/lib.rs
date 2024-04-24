pub mod matrix;
pub mod scalar;
pub mod vector;

pub trait Element:
    core::fmt::Debug + std::iter::Sum + num::Float + PartialOrd + Clone + Copy
{
}
impl<T: core::fmt::Debug + num::Float + PartialOrd + std::iter::Sum + Clone + Copy> Element for T {}

#[cfg(test)]
mod test_matrix {
    use approx::assert_relative_eq;

    use crate::{
        matrix::{Matrix2x2, Matrix3x3, Matrix4x4},
        vector::Vector3,
    };

    #[test]
    fn test_matrix_basic() {
        let _m1 = Matrix3x3::from([1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32]);

        let v2 = Vector3::from([1f32, 2f32, 3f32]);
        let m2 = Matrix3x3::from([v2, v2.mul_scalar(4f32), v2]);
        assert_eq!(m2, m2);
        assert_eq!(m2[0][0], 1f32);
    }

    #[test]
    fn test_matrix_add() {
        let m1 = Matrix3x3::from([1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32]);
        let m2 = Matrix3x3::from([9f32, 8f32, 7f32, 6f32, 5f32, 4f32, 3f32, 2f32, 1f32]);
        assert_eq!(m1.add(m2), Matrix3x3::from([10f32; 9]));
        assert_eq!(
            m1.sub(m2),
            Matrix3x3::from([-8f32, -6f32, -4f32, -2f32, 0f32, 2f32, 4f32, 6f32, 8f32])
        );
    }

    #[test]
    fn test_matrix_mul_div_scalar() {
        let data1 = [1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32];
        let m1 = Matrix3x3::from(data1);
        let scalar = 0.5f32;
        assert_eq!(
            m1.mul_scalar(scalar),
            Matrix3x3::from(data1.map(|ca| ca / 2f32))
        );
        assert_eq!(
            m1.div_scalar(scalar),
            Matrix3x3::from(data1.map(|ca| ca * 2f32))
        );
        assert_eq!(m1.mul_scalar(scalar).div_scalar(scalar), m1);
        assert_eq!(m1.div_scalar(scalar).mul_scalar(scalar), m1);
    }

    #[test]
    fn test_matrix_det() {
        let m2x2 = Matrix2x2::from([1f32, 2f32, 3f32, 4f32]);
        let m3x3 = Matrix3x3::from([-1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32]);
        let m4x4 = Matrix4x4::from([
            1.0, 3.0, 5.0, 9.0, 1.0, 3.0, 1.0, 7.0, 4.0, 3.0, 9.0, 7.0, 5.0, 2.0, 0.0, 9.0,
        ]);

        assert_relative_eq!(m2x2.det(), -2f32);
        assert_relative_eq!(m3x3.det(), 6f32);
        assert_relative_eq!(m4x4.det(), -376f32);
    }

    #[test]
    fn test_matrix_transpose() {
        let m1 = Matrix3x3::from([-1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32]);
        assert_eq!(
            m1.transpose(),
            Matrix3x3::from([-1f32, 4f32, 7f32, 2f32, 5f32, 8f32, 3f32, 6f32, 9f32,])
        )
    }
}
#[cfg(test)]
mod test_vector {
    use std::f32::consts::PI;

    use approx::assert_relative_eq;

    use super::vector::*;
    // use super::*;
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

    #[test]
    fn test_vectors_dist_angle() {
        let theta = PI / 4f32;
        let opposite_theta = theta + PI;
        let perp_theta = theta + PI / 2f32;

        let regular = Vector2::from([theta.cos(), theta.sin()]);
        let opposite = Vector2::from([opposite_theta.cos(), opposite_theta.sin()]);
        let perp = Vector2::from([perp_theta.cos(), perp_theta.sin()]);

        assert_relative_eq!(regular.dist(regular), 0f32);
        assert_relative_eq!(regular.angle(regular), 0f32);

        assert_relative_eq!(regular.dist(opposite), regular.len() * 2f32);
        assert_relative_eq!(regular.angle(opposite), PI);

        assert_relative_eq!(regular.dist(perp), 2f32.sqrt());
        assert_relative_eq!(regular.angle(perp), PI / 2f32);
    }

    #[test]
    fn test_vectors_lerp() {
        let v1 = Vector2::from([1f32, 1f32]);
        let v2 = Vector2::from([3f32, 3f32]);

        assert_eq!(v1.lerp(v2, 0f32), v1);
        assert_eq!(v1.lerp(v2, 1f32), v2);
        assert_eq!(v1.lerp(v2, 0.5f32), Vector2::from([2f32, 2f32]));
    }
}
