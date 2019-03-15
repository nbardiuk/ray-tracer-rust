use std::ops::Index;
use std::ops::Mul;
use tuples::{tuple, Tuple};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
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
    (a - b).abs() <= 1e-5
}

pub fn matrix(args: &[&[f64]]) -> Matrix {
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
        &self * &other
    }
}

impl<'a> Mul for &'a Matrix {
    type Output = Matrix;
    fn mul(self, other: &'a Matrix) -> Matrix {
        let rows = &self.data;
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
        &self * &other
    }
}

impl<'a> Mul<&'a Tuple> for &'a Matrix {
    type Output = Tuple;
    fn mul(self, other: &'a Tuple) -> Tuple {
        let rows = &self.data;
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
    pub fn transpose(&self) -> Matrix {
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

    pub fn inverse(&self) -> Matrix {
        let det = self.determinant();
        let ct = self.cofactors().transpose();
        let h = ct.data[0].len();
        let w = ct.data.len();
        let mut data = vec![vec![0.; h]; w];
        for i in 0..w {
            for j in 0..h {
                data[i][j] = ct.data[i][j] / det;
            }
        }
        Matrix { data }
    }

    fn determinant(&self) -> f64 {
        if self.data.len() == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            let mut det = 0.;
            for column in 0..self.data.len() {
                det += self.data[0][column] * self.cofactor(0, column);
            }
            det
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut data = self.data.clone();
        data.remove(row);
        for r in &mut data {
            r.remove(col);
        }
        Matrix { data: data }
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1. } else { -1. };
        sign * self.minor(row, col)
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    fn cofactors(&self) -> Matrix {
        let h = self.data[0].len();
        let w = self.data.len();
        let mut data = vec![vec![0.; h]; w];
        for i in 0..w {
            for j in 0..h {
                data[i][j] = self.cofactor(i, j);
            }
        }
        Matrix { data }
    }
}

pub fn identity_matrix() -> Matrix {
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
        assert_eq!(&a * &identity_matrix(), a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = tuple(1., 2., 3., 4.);
        assert_eq!(&identity_matrix() * &a, a);
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

    #[test]
    fn calculating_the_determinant_of_2x2_matrix() {
        let a = matrix(&[&[1., 5.], &[-3., 2.]]);
        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = matrix(&[&[1., 5., 0.], &[-3., 2., 7.], &[0., 6., -3.]]);
        let s = matrix(&[&[-3., 2.], &[0., 6.]]);
        assert_eq!(a.submatrix(0, 2), s);
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = matrix(&[
            &[-6., 1., 1., 6.],
            &[-8., 5., 8., 6.],
            &[-1., 0., 8., 2.],
            &[-7., 1., -1., 1.],
        ]);
        let s = matrix(&[&[-6., 1., 6.], &[-8., 8., 6.], &[-7., -1., 1.]]);
        assert_eq!(a.submatrix(2, 1), s);
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = matrix(&[&[3., 5., 0.], &[2., -1., -7.], &[6., -1., 5.]]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = matrix(&[&[3., 5., 0.], &[2., -1., -7.], &[6., -1., 5.]]);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn calculating_the_determinant_of_3x3_matrix() {
        let a = matrix(&[&[1., 2., 6.], &[-5., 8., -4.], &[2., 6., 4.]]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn calculating_the_determinant_of_4x4_matrix() {
        let a = matrix(&[
            &[-2., -8., 3., 5.],
            &[-3., 1., 7., 3.],
            &[1., 2., -9., 6.],
            &[-6., 7., 7., -9.],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = matrix(&[
            &[6., 4., 4., 4.],
            &[5., 5., 7., 6.],
            &[4., -9., 3., -7.],
            &[9., 1., 7., -6.],
        ]);
        assert_eq!(a.determinant(), -2120.);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_a_non_invertible_matrix_for_invertibility() {
        let a = matrix(&[
            &[-4., 2., -2., -3.],
            &[9., 6., 2., 6.],
            &[0., -5., 1., -5.],
            &[0., 0., 0., 0.],
        ]);
        assert_eq!(a.determinant(), 0.);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = matrix(&[
            &[-5., 2., 6., -8.],
            &[1., -5., 1., 8.],
            &[7., 7., -6., -7.],
            &[1., -3., 7., 4.],
        ]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b[(3, 2)], -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b[(2, 3)], 105. / 532.);
        assert_eq!(
            b,
            matrix(&[
                &[0.21805, 0.45113, 0.24060, -0.04511],
                &[-0.80827, -1.45677, -0.44361, 0.52068],
                &[-0.07895, -0.22368, -0.05263, 0.19737],
                &[-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = matrix(&[
            &[8., -5., 9., 2.],
            &[7., 5., 6., 1.],
            &[-6., 0., 9., 6.],
            &[-3., 0., -9., -4.],
        ]);
        assert_eq!(
            a.inverse(),
            matrix(&[
                &[-0.15385, -0.15385, -0.28205, -0.53846],
                &[-0.07692, 0.12308, 0.02564, 0.03077],
                &[0.35897, 0.35897, 0.43590, 0.92308],
                &[-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_third_matrix() {
        let a = matrix(&[
            &[9., 3., 0., 9.],
            &[-5., -2., -6., -3.],
            &[-4., 9., 6., 4.],
            &[-7., 6., 6., 2.],
        ]);
        assert_eq!(
            a.inverse(),
            matrix(&[
                &[-0.04074, -0.07778, 0.14444, -0.22222],
                &[-0.07778, 0.033333, 0.36667, -0.33333],
                &[-0.02901, -0.14630, -0.10926, 0.12963],
                &[0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = matrix(&[
            &[3., -9., 7., 3.],
            &[3., -8., 2., -9.],
            &[-4., 4., 4., 1.],
            &[-6., 5., -1., 1.],
        ]);
        let b = matrix(&[
            &[9., 3., 0., 9.],
            &[-5., -2., -6., -3.],
            &[-4., 9., 6., 4.],
            &[-7., 6., 6., 2.],
        ]);
        assert_eq!((&a * &b) * b.inverse(), a);
    }
}
