use matrices::Matrix;
use rays::Ray;
use std::ops::Add;
use tuples::point;
use tuples::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Bounds {
    min: Tuple,
    max: Tuple,
}

pub fn bound(min: Tuple, max: Tuple) -> Bounds {
    Bounds { min, max }
}
pub fn bound_single(p: Tuple) -> Bounds {
    bound(p.clone(), p.clone())
}

fn check_axis(origin: f64, direction: f64, minimum: f64, maximum: f64) -> (f64, f64) {
    let tmin_numerator = minimum - origin;
    let tmax_numerator = maximum - origin;
    let (tmin, tmax) = (tmin_numerator / direction, tmax_numerator / direction);
    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}
impl Bounds {
    pub fn intersects(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);
        let tmin = vec![xtmin, ytmin, ztmin]
            .into_iter()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let tmax = vec![xtmax, ytmax, ztmax]
            .into_iter()
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        tmin <= tmax && (tmin >= 0. || tmax >= 0.)
    }
    pub fn transform(&self, transform: &Matrix) -> Bounds {
        let bounds: Vec<Bounds> = vec![
            point(self.min.x, self.min.y, self.min.z),
            point(self.min.x, self.max.y, self.min.z),
            point(self.min.x, self.min.y, self.max.z),
            point(self.min.x, self.max.y, self.max.z),
            point(self.max.x, self.min.y, self.min.z),
            point(self.max.x, self.max.y, self.min.z),
            point(self.max.x, self.min.y, self.max.z),
            point(self.max.x, self.max.y, self.max.z),
        ]
        .into_iter()
        .map(|p| bound_single(transform * &p))
        .collect();

        //unsafe sum
        let mut i = bounds.into_iter();
        let first = i.next().unwrap();
        i.fold(first, |acc, b| acc + b)
    }
}

impl Add for Bounds {
    type Output = Bounds;

    fn add(self, other: Bounds) -> Bounds {
        bound(
            point(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            point(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use std::f64::consts::PI;
    use transformations::rotation_x;
    use transformations::rotation_y;
    use transformations::rotation_z;
    use tuples::vector;

    #[test]
    fn bound_is_a_pair_of_points() {
        let bound = bound(point(1., 1., 1.), point(2., 2., 2.));

        assert_eq!(bound.min, point(1., 1., 1.));
        assert_eq!(bound.max, point(2., 2., 2.));
    }

    #[test]
    fn join_bounds_into_one() {
        let a = bound(point(-1., 1., 2.), point(2., 3., 4.));
        let b = bound(point(10., 20., -30.), point(11., 22., 31.));

        let c = a + b;

        assert_eq!(c.min, point(-1., 1., -30.));
        assert_eq!(c.max, point(11., 22., 31.));
    }

    #[test]
    fn bound_transformation() {
        let square = bound(point(-1., -1., -1.), point(1., 1., 1.));
        let sq2 = 2.0_f64.sqrt();
        let on_edge = PI / 4.;
        assert_eq!(
            square.transform(&rotation_x(on_edge)),
            bound(point(-1., -sq2, -sq2), point(1., sq2, sq2))
        );
        assert_eq!(
            square.transform(&rotation_y(on_edge)),
            bound(point(-sq2, -1., -sq2), point(sq2, 1., sq2))
        );
        assert_eq!(
            square.transform(&rotation_z(on_edge)),
            bound(point(-sq2, -sq2, -1.), point(sq2, sq2, 1.))
        );
    }
    #[test]
    fn bounds_intersection() {
        let b = bound(point(-1., -1., -1.), point(1., 1., 1.));
        for (origin, direction, intersects) in vec![
            (point(0., 0., 0.), vector(1., 1., 1.), true), // inside diagonal
            (point(2., 2., 2.), vector(1., 1., 1.), false), // outside out
            (point(0., 2., 0.), vector(0., -1., 0.), true), // outside in
            (point(0., 2., 0.), vector(-1., -1., 0.), true), // outside in
            (point(0., 1.1, 0.), vector(1., 1., 0.), false), // outside parallel
            (point(0., 1.1, 0.), vector(0., 1., 1.), false), // outside parallel
            (point(0., 1.1, 0.), vector(1., 0., 1.), false), // outside parallel
        ] {
            let r = ray(origin.clone(), direction.clone());

            assert_eq!(
                b.intersects(&r),
                intersects,
                "where ray {:?} {:?}",
                origin,
                direction
            );
        }
    }
}
