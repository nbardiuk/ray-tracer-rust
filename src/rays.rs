use tuples::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}

#[cfg(test)]
mod spec {
    use super::*;
    use tuples::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1., 2., 3.);
        let direction = vector(4., 5., 6.);

        let r = ray(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = ray(point(2., 3., 4.), vector(1., 0., 0.));
        assert_eq!(r.position(0.), point(2., 3., 4.));
        assert_eq!(r.position(1.), point(3., 3., 4.));
        assert_eq!(r.position(-1.), point(1., 3., 4.));
        assert_eq!(r.position(2.5), point(4.5, 3., 4.));
    }
}
