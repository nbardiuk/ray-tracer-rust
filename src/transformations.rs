use matrices::{identity_matrix, Matrix};

fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][3] = x;
    data[1][3] = y;
    data[2][3] = z;
    Matrix { data }
}

fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][0] = x;
    data[1][1] = y;
    data[2][2] = z;
    Matrix { data }
}

#[cfg(test)]
mod spec {
    use super::*;
    use tuples::{point, vector};

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
        assert_eq!(transform * v, v);
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
}
