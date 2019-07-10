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
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;
use std::sync::Arc;
use tuples::point;
use tuples::vector;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Cone {
    pub invtransform: Matrix,
    pub material: Material,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

fn check_cap(ray: &Ray, t: f64, r: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x.powi(2) + z.powi(2) <= r.powi(2)
}
impl Cone {
    fn intersect_caps(&self, rc: Arc<Shape>, ray: &Ray) -> Vec<Intersection> {
        if !self.closed || ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            vec![self.minimum, self.maximum]
                .into_iter()
                .filter_map(|m| {
                    let t = (m - ray.origin.y) / ray.direction.y;
                    if check_cap(ray, t, m) {
                        Some(t)
                    } else {
                        None
                    }
                })
                .map(|t| intersection(t, rc.clone()))
                .collect()
        }
    }
    fn intersect_sides(&self, rc: Arc<Shape>, ray: &Ray) -> Vec<Intersection> {
        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;
        let ox = ray.origin.x;
        let oy = ray.origin.y;
        let oz = ray.origin.z;

        let a = dx.powi(2) - dy.powi(2) + dz.powi(2);
        let b = 2. * (ox * dx - oy * dy + oz * dz);
        let c = ox.powi(2) - oy.powi(2) + oz.powi(2);
        let discriminant = b.powi(2) - 4. * a * c;

        if a.abs() < EPSILON && EPSILON < b.abs() {
            vec![intersection(-c / (2. * b), rc.clone())]
        } else if discriminant < 0. {
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

impl Shape for Cone {
    fn local_bounds(&self) -> Bounds {
        bound(
            point(self.minimum, self.minimum, self.minimum),
            point(self.maximum, self.maximum, self.maximum),
        )
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
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1. && point.y >= self.maximum - EPSILON {
            vector(0., 1., 0.)
        } else if dist < 1. && point.y <= self.minimum + EPSILON {
            vector(0., -1., 0.)
        } else {
            let y = dist.sqrt();
            let y = if point.y > 0. { -y } else { y };
            vector(point.x, y, point.z)
        }
    }
    fn local_intersects(&self, rc: Arc<Shape>, ray: Ray) -> Vec<Intersection> {
        let sides = self.intersect_sides(rc.clone(), &ray);
        let caps = self.intersect_caps(rc.clone(), &ray);
        sides.into_iter().chain(caps.into_iter()).collect()
    }
}
pub fn cone() -> Cone {
    let material = material();
    let invtransform = identity_matrix();
    Cone {
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
    fn intersecting_a_cone_with_a_ray() {
        let c = Arc::new(cone());
        for (origin, direction, t0, t1) in vec![
            (point(0., 0., -5.), vector(0., 0., 1.), 5., 5.),
            (point(0., 0., -5.), vector(1., 1., 1.), 8.66025, 8.66025),
            (point(1., 1., -5.), vector(-0.5, -1., 1.), 4.55006, 49.44994),
        ] {
            let xs = c.local_intersects(c.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), 2);
            assert_that!(xs[0].t, close_to(t0, 10e-5));
            assert_that!(xs[1].t, close_to(t1, 10e-5));
        }
    }
    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let c = Arc::new(cone());

        let xs = c.local_intersects(
            c.clone(),
            ray(point(0., 0., -1.), vector(0., 1., 1.).normalized()),
        );

        assert_eq!(xs.len(), 1);
        assert_that!(xs[0].t, close_to(0.35355, 10e-5));
    }
    #[test]
    fn intersecting_a_cones_end_caps() {
        let mut c = cone();
        c.minimum = -0.5;
        c.maximum = 0.5;
        c.closed = true;
        let c = Arc::new(c);
        for (origin, direction, count) in vec![
            (point(0., 0., -5.), vector(0., 1., 0.), 0),
            (point(0., 0., -0.25), vector(0., 1., 1.), 2),
            (point(0., 0., -0.25), vector(0., 1., 0.), 4),
        ] {
            let xs = c.local_intersects(c.clone(), ray(origin, direction.normalized()));

            assert_eq!(xs.len(), count);
        }
    }
    #[test]
    fn normal_vector_on_a_cone() {
        let c = cone();
        for (point, normal) in vec![
            (point(0., 0., 0.), vector(0., 0., 0.)),
            (point(1., 1., 1.), vector(1., -(2. as f64).sqrt(), 1.)),
            (point(-1., -1., 0.), vector(-1., 1., 0.)),
        ] {
            assert_eq!(c.local_normal_at(point), normal);
        }
    }
    #[test]
    fn a_bounds_of_a_cone() {
        let c = cone();

        assert_eq!(
            c.local_bounds(),
            bound(
                point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
                point(INFINITY, INFINITY, INFINITY)
            )
        );
    }
    #[test]
    fn a_bounds_of_a_bounded_cone() {
        let mut c = cone();
        c.minimum = -2.;
        c.maximum = 4.;

        assert_eq!(
            c.local_bounds(),
            bound(point(-2., -2., -2.), point(4., 4., 4.))
        );
    }
}
