use matrices::Matrix;
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

impl Bounds {
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
    use std::f64::consts::PI;
    use transformations::rotation_x;
    use transformations::rotation_y;
    use transformations::rotation_z;

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
}
