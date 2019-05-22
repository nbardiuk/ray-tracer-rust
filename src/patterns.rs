use matrices::identity_matrix;
use matrices::Matrix;
use shapes::Shape;
use std::rc::Rc;
use tuples::Color;
use tuples::Tuple;

pub trait Pattern {
    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);

    fn at(&self, point: &Tuple) -> Color;
    fn at_shape(&self, shape: Rc<Shape>, world_point: &Tuple) -> Color {
        let shape_point = (&shape.transform().inverse()) * world_point;
        let pattern_point = self.transform().inverse() * shape_point;
        self.at(&pattern_point)
    }
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pattern ({:?})", self.transform())
    }
}

impl PartialEq<Pattern> for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        self.transform().eq(other.transform())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Pattern for Stripe {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
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
    let transform = identity_matrix();
    Stripe { a, b, transform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Gradient {
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for Gradient {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn at(&self, point: &Tuple) -> Color {
        let distance = &self.b - &self.a;
        let fraction = point.x - point.x.floor();
        &self.a + &(distance * fraction)
    }
}
pub fn gradient_pattern(a: Color, b: Color) -> Gradient {
    let transform = identity_matrix();
    Gradient { a, b, transform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ring {
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for Ring {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
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
    let transform = identity_matrix();
    Ring { a, b, transform }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Checkers {
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for Checkers {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
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
    let transform = identity_matrix();
    Checkers { a, b, transform }
}

#[cfg(test)]
mod spec {
    use super::*;
    use spheres::sphere;
    use transformations::scaling;
    use transformations::translation;
    use tuples::color;
    use tuples::point;
    use tuples::Color;

    fn black() -> Color {
        color(0., 0., 0.)
    }
    fn white() -> Color {
        color(1., 1., 1.)
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
        object.transform = scaling(2., 2., 2.);
        let pattern = stripe_pattern(white(), black());

        let c = pattern.at_shape(Rc::new(object), &point(1.5, 0., 0.));

        assert_eq!(c, white());
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = stripe_pattern(white(), black());
        pattern.set_transform(scaling(2., 2., 2.));

        let c = pattern.at_shape(Rc::new(object), &point(1.5, 0., 0.));

        assert_eq!(c, white());
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.transform = scaling(2., 2., 2.);
        let mut pattern = stripe_pattern(white(), black());
        pattern.set_transform(translation(0.5, 0., 0.));

        let c = pattern.at_shape(Rc::new(object), &point(2.5, 0., 0.));

        assert_eq!(c, white());
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