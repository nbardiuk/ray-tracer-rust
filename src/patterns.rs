use tuples::Color;
use tuples::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn at(&self, point: &Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Stripe {
    Stripe { a, b }
}

#[cfg(test)]
mod spec {
    use super::*;
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
}
