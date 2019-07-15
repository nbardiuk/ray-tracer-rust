use crate::matrices::Matrix;
use crate::tuples::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        &self.origin + &self.direction * time
    }

    pub fn transform(&self, m: &Matrix) -> Ray {
        ray(m * &self.origin, m * &self.direction)
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}

#[cfg(test)]
mod spec {
    use super::*;
    use crate::transformations::{scaling, translation};
    use crate::tuples::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1., 2., 3.);
        let direction = vector(4., 5., 6.);

        let r = ray(origin.clone(), direction.clone());

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

    #[test]
    fn translating_a_ray() {
        let r = ray(point(1., 2., 3.), vector(0., 1., 0.));
        let m = translation(3., 4., 5.);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, point(4., 6., 8.));
        assert_eq!(r2.direction, vector(0., 1., 0.));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1., 2., 3.), vector(0., 1., 0.));
        let m = scaling(2., 3., 4.);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, point(2., 6., 12.));
        assert_eq!(r2.direction, vector(0., 3., 0.));
    }
}
