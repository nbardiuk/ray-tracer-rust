use matrices::{identity_matrix, Matrix};

fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut data = identity_matrix().data;
    data[0][3] = x;
    data[1][3] = y;
    data[2][3] = z;
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
}
