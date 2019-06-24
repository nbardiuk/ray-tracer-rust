use intersections::intersection;
use intersections::Intersection;
use intersections::EPSILON;
use materials::material;
use materials::Material;
use matrices::identity_matrix;
use matrices::Matrix;
use rays::Ray;
use shapes::Shape;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;
use std::rc::Rc;
use tuples::vector;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Cylinder {
    pub invtransform: Matrix,
    pub material: Material,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x.powi(2) + z.powi(2) <= 1.
}
impl Cylinder {
    fn intersect_caps(&self, rc: Rc<Shape>, ray: &Ray) -> Vec<Intersection> {
        if !self.closed || ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            vec![self.minimum, self.maximum]
                .into_iter()
                .map(|m| (m - ray.origin.y) / ray.direction.y)
                .filter(|t| check_cap(ray, *t))
                .map(|t| intersection(t, rc.clone()))
                .collect()
        }
    }
    fn intersect_sides(&self, rc: Rc<Shape>, ray: &Ray) -> Vec<Intersection> {
        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;
        let ox = ray.origin.x;
        let oy = ray.origin.y;
        let oz = ray.origin.z;

        let a = dx.powi(2) + dz.powi(2);
        let b = 2. * (ox * dx + oz * dz);
        let c = ox.powi(2) + oz.powi(2) - 1.;
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2. * a),
                (-b + discriminant.sqrt()) / (2. * a),
            ]
            .into_iter()
            .filter(|t| {
                let y = oy + t * dy;
                self.minimum < y && y < self.maximum
            })
            .map(|t| intersection(t, rc.clone()))
            .collect()
        }
    }
}

impl Shape for Cylinder {
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
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1. && point.y >= self.maximum - EPSILON {
            vector(0., 1., 0.)
        } else if dist < 1. && point.y <= self.minimum + EPSILON {
            vector(0., -1., 0.)
        } else {
            vector(point.x, 0., point.z)
        }
    }
    fn local_intersects(&self, rc: Rc<Shape>, ray: Ray) -> Vec<Intersection> {
        let sides = self.intersect_sides(rc.clone(), &ray);
        let caps = self.intersect_caps(rc.clone(), &ray);
        sides.into_iter().chain(caps.into_iter()).collect()
    }
}
pub fn cylinder() -> Cylinder {
    let material = material();
    let invtransform = identity_matrix();
    Cylinder {
        material,
        invtransform,
        minimum: NEG_INFINITY,
        maximum: INFINITY,
        closed: false,
    }
}
#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use rays::ray;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Rc::new(cylinder());
        for (origin, direction) in vec![
            (point(1., 0., 0.), vector(0., 1., 0.)),
            (point(0., 0., 0.), vector(0., 1., 0.)),
            (point(0., 0., -5.), vector(1., 1., 1.)),
        ] {
            let xs = cyl.local_intersects(cyl.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), 0);
        }
    }
    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Rc::new(cylinder());
        for (origin, direction, t0, t1) in vec![
            (point(1., 0., -5.), vector(0., 0., 1.), 5., 5.),
            (point(0., 0., -5.), vector(0., 0., 1.), 4., 6.),
            (point(0.5, 0., -5.), vector(0.1, 1., 1.), 6.80798, 7.08872),
        ] {
            let xs = cyl.local_intersects(cyl.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), 2);
            assert_that!(xs[0].t, close_to(t0, 10e-5));
            assert_that!(xs[1].t, close_to(t1, 10e-5));
        }
    }
    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = cylinder();
        for (point, normal) in vec![
            (point(1., 0., 0.), vector(1., 0., 0.)),
            (point(0., 5., -1.), vector(0., 0., -1.)),
            (point(0., -2., 1.), vector(0., 0., 1.)),
            (point(-1., 1., 0.), vector(-1., 0., 0.)),
        ] {
            assert_eq!(cyl.local_normal_at(point), normal);
        }
    }
    #[test]
    fn intersecting_a_constrained_cylinder() {
        let mut cyl = cylinder();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        let cyl = Rc::new(cyl);
        for (origin, direction, count) in vec![
            (point(0., 1.5, 0.), vector(0.1, 1., 0.), 0),
            (point(0., 3., -5.), vector(0., 0., 1.), 0),
            (point(0., 0., -5.), vector(0., 0., 1.), 0),
            (point(0., 2., -5.), vector(0., 0., 1.), 0),
            (point(0., 1., -5.), vector(0., 0., 1.), 0),
            (point(0., 1.5, -2.), vector(0., 0., 1.), 2),
        ] {
            let xs = cyl.local_intersects(cyl.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), count);
        }
    }
    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = cylinder();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;
        let cyl = Rc::new(cyl);
        for (origin, direction, count) in vec![
            (point(0., 3., 0.), vector(0., -1., 0.), 2),
            (point(0., 3., -2.), vector(0., -1., 2.), 2),
            (point(0., 4., -2.), vector(0., -1., 1.), 2),
            (point(0., 0., -2.), vector(0., 1., 2.), 2),
            (point(0., -1., -2.), vector(0., 1., 1.), 2),
        ] {
            let xs = cyl.local_intersects(cyl.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), count);
        }
    }
    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let mut cyl = cylinder();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;
        for (point, normal) in vec![
            (point(0., 1., 0.), vector(0., -1., 0.)),
            (point(0.5, 1., 0.), vector(0., -1., 0.)),
            (point(0., 1., 0.5), vector(0., -1., 0.)),
            (point(0., 2., 0.), vector(0., 1., 0.)),
            (point(0.5, 2., 0.), vector(0., 1., 0.)),
            (point(0., 2., 0.5), vector(0., 1., 0.)),
        ] {
            assert_eq!(cyl.local_normal_at(point), normal);
        }
    }
}
