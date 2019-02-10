use std::ops::Index;

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<f64>>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(l, r)| l.iter().zip(r.iter()).all(|(a, b)| close(*a, *b)))
    }
}
fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-7
}

fn matrix(args: &[&[f64]]) -> Matrix {
    let data = args
        .iter()
        .map(|row| Vec::from(*row))
        .collect::<Vec<Vec<f64>>>();
    Matrix { data }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, pair: (usize, usize)) -> &f64 {
        &self.data[pair.0][pair.1]
    }
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = matrix(&[
            &[1., 2., 3., 4.],
            &[5.5, 6.5, 7.5, 8.5],
            &[9., 10., 11., 12.],
            &[13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 3)], 4.);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = matrix(&[&[-3., 5.], &[1., -2.]]);

        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(0, 1)], 5.);
        assert_eq!(m[(1, 0)], 1.);
        assert_eq!(m[(1, 1)], -2.);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = matrix(&[&[-3., 5., 0.], &[1., -2., -7.], &[0., 1., 1.]]);

        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(1, 1)], -2.);
        assert_eq!(m[(2, 2)], 1.);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = matrix(&[
            &[1., 2., 3., 4.],
            &[5., 6., 7., 8.],
            &[9., 8., 7., 6.],
            &[5., 4., 3., 2.],
        ]);
        let b = matrix(&[
            &[1., 2., 3., 4.],
            &[5., 6., 7., 8.],
            &[9., 8., 7., 6.],
            &[5., 4., 3., 2.],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix(&[
            &[1., 2., 3., 4.],
            &[5., 6., 7., 8.],
            &[9., 8., 7., 6.],
            &[5., 4., 3., 2.],
        ]);
        let b = matrix(&[
            &[2., 3., 4., 5.],
            &[6., 7., 8., 9.],
            &[8., 7., 6., 5.],
            &[4., 3., 2., 1.],
        ]);
        assert_ne!(a, b);
    }
}
