use matrices::identity_matrix;
use matrices::Matrix;
use shapes::Shape;
use tuples::Color;
use tuples::Tuple;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Stripe {
    pub fn at(&self, point: &Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn at_object(&self, object: Rc<Shape>, world_point: &Tuple) -> Color {
        let object_point = (&object.transform().inverse()) * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.at(&pattern_point)
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Stripe {
    let transform = identity_matrix();
    Stripe { a, b, transform }
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
    fn stripes_with_an_object_transformation() {
        let mut object = sphere();
        object.transform = scaling(2., 2., 2.);
        let pattern = stripe_pattern(white(), black());

        let c = pattern.at_object(Rc::new(object), &point(1.5, 0., 0.));

        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = stripe_pattern(white(), black());
        pattern.transform = scaling(2., 2., 2.);

        let c = pattern.at_object(Rc::new(object), &point(1.5, 0., 0.));

        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.transform = scaling(2., 2., 2.);
        let mut pattern = stripe_pattern(white(), black());
        pattern.transform = translation(0.5, 0., 0.);

        let c = pattern.at_object(Rc::new(object), &point(2.5, 0., 0.));

        assert_eq!(c, white());
    }
}
