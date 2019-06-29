use bounds::bound;
use bounds::Bounds;
use intersections::intersection;
use intersections::Intersection;
use intersections::EPSILON;
use materials::material;
use materials::Material;
use matrices::identity_matrix;
use matrices::Matrix;
use rays::Ray;
use shapes::Shape;
use std::rc::Rc;
use tuples::point;
use tuples::vector;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Cube {
    pub invtransform: Matrix,
    pub material: Material,
    bounds: Bounds,
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1. - origin;
    let tmax_numerator = 1. - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        let inifinity: f64 = 10e100;
        (tmin_numerator * inifinity, tmax_numerator * inifinity)
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Shape for Cube {
    fn local_bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn material(&self) -> &Material {
        &self.material
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }
    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }
    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let comps = [point.x.abs(), point.y.abs(), point.z.abs()];
        let maxc = *comps
            .iter()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        if maxc == comps[0] {
            vector(point.x, 0., 0.)
        } else if maxc == comps[1] {
            vector(0., point.y, 0.)
        } else {
            vector(0., 0., point.z)
        }
    }
    fn local_intersects(&self, rc: Rc<Shape>, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);
        let mins = [xtmin, ytmin, ztmin];
        let maxs = [xtmax, ytmax, ztmax];
        let tmin = mins
            .iter()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let tmax = maxs
            .iter()
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        if tmin > tmax {
            vec![]
        } else {
            vec![
                intersection(*tmin, rc.clone()),
                intersection(*tmax, rc.clone()),
            ]
        }
    }
}

pub fn cube() -> Cube {
    let material = material();
    let invtransform = identity_matrix();
    Cube {
        material,
        invtransform,
        bounds: bound(point(-1., -1., -1.), point(1., 1., 1.)),
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Rc::new(cube());
        for (origin, direction, t1, t2) in vec![
            (point(5., 0.5, 0.), vector(-1., 0., 0.), 4., 6.),
            (point(-5., 0.5, 0.), vector(1., 0., 0.), 4., 6.),
            (point(0.5, 5., 0.), vector(0., -1., 0.), 4., 6.),
            (point(0.5, -5., 0.), vector(0., 1., 0.), 4., 6.),
            (point(0.5, 0., 5.), vector(0., 0., -1.), 4., 6.),
            (point(0.5, 0., -5.), vector(0., 0., 1.), 4., 6.),
        ] {
            let xs = c.local_intersects(c.clone(), ray(origin, direction));

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t1);
            assert_eq!(xs[1].t, t2);
        }
    }
    #[test]
    fn a_ray_misses_a_cube() {
        let c = Rc::new(cube());
        for (origin, direction) in vec![
            (point(-2., 0., 0.), vector(0.2673, 0.5345, 0.8018)),
            (point(0., -2., 0.), vector(0.8018, 0.2673, 0.5345)),
            (point(0., 0., -2.), vector(0.5345, 0.8018, 0.2673)),
            (point(2., 0., 2.), vector(0., 0., -1.)),
            (point(0., 2., 2.), vector(0., -1., 0.)),
            (point(2., 2., 0.), vector(-1., 0., 0.)),
        ] {
            let xs = c.local_intersects(c.clone(), ray(origin, direction));

            assert_eq!(xs.len(), 0);
        }
    }
    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Rc::new(cube());
        for (point, normal) in vec![
            (point(1., 0.5, -0.8), vector(1., 0., 0.)),
            (point(-1., -0.2, 0.9), vector(-1., 0., 0.)),
            (point(-0.4, 1., -0.1), vector(0., 1., 0.)),
            (point(0.3, -1., -0.7), vector(0., -1., 0.)),
            (point(-0.6, 0.3, 1.), vector(0., 0., 1.)),
            (point(0.4, 0.4, -1.), vector(0., 0., -1.)),
            (point(1., 1., 1.), vector(1., 0., 0.)),
            (point(-1., -1., -1.), vector(-1., 0., 0.)),
        ] {
            assert_eq!(c.local_normal_at(point), normal);
        }
    }
    #[test]
    fn a_bounds_of_a_cube() {
        let c = cube();

        assert_eq!(
            c.local_bounds(),
            bound(point(-1., -1., -1.), point(1., 1., 1.))
        );
    }
}
