use crate::matrices::identity_matrix;
use crate::matrices::Matrix;
use crate::shapes::SyncShape;
use crate::tuples::Color;
use crate::tuples::Tuple;
use std::sync::Arc;

pub type SyncPattern = dyn Pattern + Sync + Send;

pub trait Pattern {
    fn invtransform(&self) -> &Matrix;
    fn set_invtransform(&mut self, invtransform: Matrix);

    fn at(&self, point: &Tuple) -> Color;
    fn at_shape(&self, shape: Arc<SyncShape>, world_point: &Tuple) -> Color {
        let shape_point = shape.world_to_object(world_point);
        let pattern_point = self.invtransform() * &shape_point;
        self.at(&pattern_point)
    }
}

impl std::fmt::Debug for SyncPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pattern ({:?})", self.invtransform())
    }
}

impl PartialEq<SyncPattern> for SyncPattern {
    fn eq(&self, other: &SyncPattern) -> bool {
        self.invtransform().eq(other.invtransform())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    invtransform: Matrix,
}

impl Pattern for Stripe {
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }

    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }

    fn at(&self, point: &Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}
pub fn stripe_pattern(a: Color, b: Color) -> Stripe {
    let invtransform = identity_matrix();
    Stripe { a, b, invtransform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Gradient {
    a: Color,
    b: Color,
    invtransform: Matrix,
}
impl Pattern for Gradient {
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }

    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }

    fn at(&self, point: &Tuple) -> Color {
        let distance = &self.b - &self.a;
        let fraction = point.x - point.x.floor();
        &self.a + &(distance * fraction)
    }
}
pub fn gradient_pattern(a: Color, b: Color) -> Gradient {
    let invtransform = identity_matrix();
    Gradient { a, b, invtransform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ring {
    a: Color,
    b: Color,
    invtransform: Matrix,
}
impl Pattern for Ring {
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }

    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }

    fn at(&self, point: &Tuple) -> Color {
        if ((point.x.powi(2) + point.z.powi(2)).sqrt()).floor() as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}
pub fn ring_pattern(a: Color, b: Color) -> Ring {
    let invtransform = identity_matrix();
    Ring { a, b, invtransform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Checkers {
    a: Color,
    b: Color,
    invtransform: Matrix,
}
impl Pattern for Checkers {
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }

    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }

    fn at(&self, point: &Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}
pub fn checkers_pattern(a: Color, b: Color) -> Checkers {
    let invtransform = identity_matrix();
    Checkers { a, b, invtransform }
}

#[cfg(test)]
pub mod spec {
    use super::*;
    use crate::spheres::sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuples::color;
    use crate::tuples::point;
    use crate::tuples::Color;

    fn black() -> Color {
        color(0., 0., 0.)
    }
    fn white() -> Color {
        color(1., 1., 1.)
    }

    pub struct TestPattern {
        invtransform: Matrix,
    }
    impl Pattern for TestPattern {
        fn invtransform(&self) -> &Matrix {
            &self.invtransform
        }

        fn set_invtransform(&mut self, invtransform: Matrix) {
            self.invtransform = invtransform;
        }

        fn at(&self, point: &Tuple) -> Color {
            color(point.x, point.y, point.z)
        }
    }
    pub fn test_pattern() -> TestPattern {
        let invtransform = identity_matrix();
        TestPattern { invtransform }
    }

    #[test]
    fn creating_a_simple_pattern() {
        let pattern = stripe_pattern(white(), black());

        assert_eq!(pattern.a, white());
        assert_eq!(pattern.b, black());
    }

    #[test]
    fn a_stripe_patter_is_constant_in_y() {
        let pattern = stripe_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0., 1., 0.)), white());
        assert_eq!(pattern.at(&point(0., 2., 0.)), white());
    }

    #[test]
    fn a_stripe_patter_is_constant_in_z() {
        let pattern = stripe_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0., 0., 1.)), white());
        assert_eq!(pattern.at(&point(0., 0., 2.)), white());
    }

    #[test]
    fn a_stripe_patter_alternates_in_x() {
        let pattern = stripe_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0.9, 0., 0.)), white());
        assert_eq!(pattern.at(&point(1., 0., 0.)), black());
        assert_eq!(pattern.at(&point(-0.1, 0., 0.)), black());
        assert_eq!(pattern.at(&point(-1.1, 0., 0.)), white());
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut object = sphere();
        object.invtransform = scaling(2., 2., 2.).inverse();
        let pattern = test_pattern();

        let c = pattern.at_shape(Arc::new(object), &point(2., 3., 4.));

        assert_eq!(c, color(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = test_pattern();
        pattern.set_invtransform(scaling(2., 2., 2.).inverse());

        let c = pattern.at_shape(Arc::new(object), &point(2., 3., 4.));

        assert_eq!(c, color(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.invtransform = scaling(2., 2., 2.).inverse();
        let mut pattern = test_pattern();
        pattern.set_invtransform(translation(0.5, 1., 1.5).inverse());

        let c = pattern.at_shape(Arc::new(object), &point(2.5, 3., 3.5));

        assert_eq!(c, color(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = gradient_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0.25, 0., 0.)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.at(&point(0.5, 0., 0.)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.at(&point(0.75, 0., 0.)), color(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = ring_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(1., 0., 0.)), black());
        assert_eq!(pattern.at(&point(0., 0., 1.)), black());
        assert_eq!(pattern.at(&point(0.708, 0., 0.708)), black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = checkers_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0.99, 0., 0.)), white());
        assert_eq!(pattern.at(&point(1.01, 0., 0.)), black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = checkers_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0., 0.99, 0.)), white());
        assert_eq!(pattern.at(&point(0., 1.01, 0.)), black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = checkers_pattern(white(), black());

        assert_eq!(pattern.at(&point(0., 0., 0.)), white());
        assert_eq!(pattern.at(&point(0., 0., 0.99)), white());
        assert_eq!(pattern.at(&point(0., 0., 1.01)), black());
    }
}
