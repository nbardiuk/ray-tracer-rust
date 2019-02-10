use std::ops::Index;
use std::ops::Mul;
use tuples::{tuple, Tuple};

#[derive(Clone, Debug)]
struct Matrix {
    data: Vec<Vec<f64>>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.data.len() == other.data.len()
            && self.data.iter().zip(other.data.iter()).all(|(l, r)| {
                l.len() == r.len() && l.iter().zip(r.iter()).all(|(a, b)| close(*a, *b))
            })
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

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        let rows = self.data;
        let cols = other.transpose().data;
        let mut data = vec![vec![0.; cols.len()]; rows.len()];
        for i in 0..rows.len() {
            for j in 0..cols.len() {
                data[i][j] = dot(&rows[i], &cols[j]);
            }
        }
        Matrix { data }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        let rows = self.data;
        let cols = matrix(&[&[other.x, other.y, other.z, other.w]]).data;
        let mut tuple = tuple(0., 0., 0., 0.);
        tuple.x = dot(&rows[0], &cols[0]);
        tuple.y = dot(&rows[1], &cols[0]);
        tuple.z = dot(&rows[2], &cols[0]);
        tuple.w = dot(&rows[3], &cols[0]);
        tuple
    }
}

fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(l, r)| l * r)
        .fold(0., |acc, f| acc + f)
}

impl Matrix {
    fn transpose(&self) -> Matrix {
        let w = self.data[0].len();
        let h = self.data.len();
        let mut data = vec![vec![0.; h]; w];
        for i in 0..w {
            for j in 0..h {
                data[i][j] = self.data[j][i];
            }
        }
        Matrix { data }
    }
}

fn identity_matrix() -> Matrix {
    matrix(&[
        &[1., 0., 0., 0.],
        &[0., 1., 0., 0.],
        &[0., 0., 1., 0.],
        &[0., 0., 0., 1.],
    ])
}

#[cfg(test)]
mod spec {
    use super::*;
    use tuples::tuple;

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

    #[test]
    fn multiplying_two_matrices() {
        let a = matrix(&[
            &[1., 2., 3., 4.],
            &[5., 6., 7., 8.],
            &[9., 8., 7., 6.],
            &[5., 4., 3., 2.],
        ]);
        let b = matrix(&[
            &[-2., 1., 2., 3.],
            &[3., 2., 1., -1.],
            &[4., 3., 6., 5.],
            &[1., 2., 7., 8.],
        ]);
        let ab = matrix(&[
            &[20., 22., 50., 48.],
            &[44., 54., 114., 108.],
            &[40., 58., 110., 102.],
            &[16., 26., 46., 42.],
        ]);
        assert_eq!(a * b, ab);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = matrix(&[
            &[1., 2., 3., 4.],
            &[2., 4., 4., 2.],
            &[8., 6., 4., 1.],
            &[0., 0., 0., 1.],
        ]);
        let b = tuple(1., 2., 3., 1.);
        assert_eq!(a * b, tuple(18., 24., 33., 1.));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = matrix(&[
            &[0., 1., 2., 4.],
            &[1., 2., 4., 8.],
            &[2., 4., 8., 16.],
            &[4., 8., 16., 32.],
        ]);
        assert_eq!(a.clone() * identity_matrix(), a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = tuple(1., 2., 3., 4.);
        assert_eq!(identity_matrix() * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = matrix(&[
            &[0., 9., 3., 0.],
            &[9., 8., 0., 8.],
            &[1., 8., 5., 3.],
            &[0., 0., 5., 8.],
        ]);
        assert_eq!(
            a.transpose(),
            matrix(&[
                &[0., 9., 1., 0.],
                &[9., 8., 8., 0.],
                &[3., 0., 5., 5.],
                &[0., 8., 3., 8.],
            ])
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = identity_matrix().transpose();
        assert_eq!(a, identity_matrix());
    }
}
