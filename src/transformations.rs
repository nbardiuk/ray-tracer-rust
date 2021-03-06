use crate::matrices::identity_matrix;
use crate::matrices::matrix;
use crate::matrices::Matrix;
use crate::tuples::Tuple;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][3] = x;
    data[1][3] = y;
    data[2][3] = z;
    Matrix { data }
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][0] = x;
    data[1][1] = y;
    data[2][2] = z;
    Matrix { data }
}

pub fn rotation_x(r: f64) -> Matrix {
    let mut data = identity_matrix().data;
    let c = r.cos();
    let s = r.sin();
    data[1][1] = c;
    data[1][2] = -s;
    data[2][1] = s;
    data[2][2] = c;
    Matrix { data }
}

pub fn rotation_y(r: f64) -> Matrix {
    let mut data = identity_matrix().data;
    let c = r.cos();
    let s = r.sin();
    data[0][0] = c;
    data[0][2] = s;
    data[2][0] = -s;
    data[2][2] = c;
    Matrix { data }
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut data = identity_matrix().data;
    let c = r.cos();
    let s = r.sin();
    data[0][0] = c;
    data[0][1] = -s;
    data[1][0] = s;
    data[1][1] = c;
    Matrix { data }
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][1] = xy;
    data[0][2] = xz;
    data[1][0] = yx;
    data[1][2] = yz;
    data[2][0] = zx;
    data[2][1] = zy;
    Matrix { data }
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (to - from).normalized();
    let left = forward.cross(&up.normalized());
    let true_up = left.cross(&forward);
    let orientation = matrix(&[
        &[left.x, left.y, left.z, 0.],
        &[true_up.x, true_up.y, true_up.z, 0.],
        &[-forward.x, -forward.y, -forward.z, 0.],
        &[0., 0., 0., 1.],
    ]);
    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod spec {
    use super::*;
    use crate::matrices::matrix;
    use crate::tuples::{point, vector};
    use std::f64::consts::PI;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let p = point(-3., 4., 5.);
        assert_eq!(transform * p, point(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let inv = transform.inverse();
        let p = point(-3., 4., 5.);
        assert_eq!(inv * p, point(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5., -3., 2.);
        let v = vector(-3., 4., 5.);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2., 3., 4.);
        let p = point(-4., 6., 8.);
        assert_eq!(transform * p, point(-8., 18., 32.));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);
        assert_eq!(transform * v, vector(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);
        assert_eq!(transform.inverse() * v, vector(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1., 1., 1.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(-2., 3., 4.));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let full_quarter = rotation_x(PI / 2.);
        assert_eq!(
            &half_quarter * &p,
            point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
        assert_eq!(&full_quarter * &p, point(0., 0., 1.));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let inv = half_quarter.inverse();
        assert_eq!(inv * p, point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = point(0., 0., 1.);
        let half_quarter = rotation_y(PI / 4.);
        let full_quarter = rotation_y(PI / 2.);
        assert_eq!(
            &half_quarter * &p,
            point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.)
        );
        assert_eq!(&full_quarter * &p, point(1., 0., 0.));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_z(PI / 4.);
        let full_quarter = rotation_z(PI / 2.);
        assert_eq!(
            &half_quarter * &p,
            point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.)
        );
        assert_eq!(&full_quarter * &p, point(-1., 0., 0.));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1., 0., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(5., 3., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0., 1., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(6., 3., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0., 0., 1., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 5., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0., 0., 0., 1., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 7., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0., 0., 0., 0., 1., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 3., 6.));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0., 0., 0., 0., 0., 1.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);

        let p = a * p;
        assert_eq!(p, point(1., -1., 0.));

        let p = b * p;
        assert_eq!(p, point(5., -5., 0.));

        let p = c * p;
        assert_eq!(p, point(15., 0., 7.));
    }

    #[test]
    fn chained_transformation_must_be_applied_in_reverse_order() {
        let p = point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);

        assert_eq!(c * b * a * p, point(15., 0., 7.));
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = point(0., 0., 0.);
        let to = point(0., 0., -1.);
        let up = point(0., 1., 0.);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, identity_matrix());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0., 0., 0.);
        let to = point(0., 0., 1.);
        let up = vector(0., 1., 0.);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, scaling(-1., 1., -1.));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = point(0., 0., 8.);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, translation(0., 0., -8.));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = point(1., 3., 2.);
        let to = point(4., -2., 8.);
        let up = vector(1., 1., 0.);

        let t = view_transform(&from, &to, &up);

        assert_eq!(
            t,
            matrix(&[
                &[-0.50709, 0.50709, 0.67612, -2.36643],
                &[0.76772, 0.60609, 0.12122, -2.82843],
                &[-0.35857, 0.59761, -0.71714, 0.],
                &[0., 0., 0., 1.],
            ])
        );
    }
}
