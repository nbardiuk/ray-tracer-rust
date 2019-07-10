use bounds::bound_vector;
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
use std::sync::Arc;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    normal: Tuple,
    pub invtransform: Matrix,
    pub material: Material,
    bounds: Bounds,
}

impl Shape for Triangle {
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
    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        self.normal.clone()
    }
    fn local_intersects(&self, rc: Arc<Shape>, ray: Ray) -> Vec<Intersection> {
        let d_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&d_e2);
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1. / det;
        let p1_or = &ray.origin - &self.p1;
        let u = f * p1_or.dot(&d_e2);
        if u < 0. || 1. < u {
            return vec![];
        }

        let o_e1 = p1_or.cross(&self.e1);
        let v = f * ray.direction.dot(&o_e1);
        if v < 0. || 1. < u + v {
            return vec![];
        }

        let t = f * self.e2.dot(&o_e1);
        vec![intersection(t, rc)]
    }
}

pub fn triangle(p1: Tuple, p2: Tuple, p3: Tuple) -> Triangle {
    let e1 = &p2 - &p1;
    let e2 = &p3 - &p1;
    let normal = e2.cross(&e1).normalized();
    let material = material();
    let invtransform = identity_matrix();
    let bounds = bound_vector(vec![p1.clone(), p2.clone(), p3.clone()]);
    Triangle {
        p1,
        p2,
        p3,
        e1,
        e2,
        normal,
        material,
        invtransform,
        bounds,
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn constucting_a_triangle() {
        let p1 = point(0., 1., 0.);
        let p2 = point(-1., 0., 0.);
        let p3 = point(1., 0., 0.);

        let t = triangle(p1.clone(), p2.clone(), p3.clone());

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, vector(-1., -1., 0.));
        assert_eq!(t.e2, vector(1., -1., 0.));
        assert_eq!(t.normal, vector(0., 0., -1.));
    }

    #[test]
    fn finding_the_noral_on_a_triangle() {
        let t = triangle(point(0., 1., 0.), point(-1., 0., 0.), point(1., 0., 0.));

        assert_eq!(t.local_normal_at(point(0., 0.5, 0.)), t.normal);
        assert_eq!(t.local_normal_at(point(-0.5, 0.75, 0.)), t.normal);
        assert_eq!(t.local_normal_at(point(-0.5, 0.25, 0.)), t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Arc::new(triangle(
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
        ));
        let r = ray(point(0., -1., -2.), vector(0., 1., 0.));

        let xs = t.local_intersects(t.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_p1_p3_edge() {
        let t = Arc::new(triangle(
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
        ));
        let r = ray(point(1., 1., -2.), vector(0., 0., 1.));

        let xs = t.local_intersects(t.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_p1_p2_edge() {
        let t = Arc::new(triangle(
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
        ));
        let r = ray(point(-1., 1., -2.), vector(0., 0., 1.));

        let xs = t.local_intersects(t.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_p2_p3_edge() {
        let t = Arc::new(triangle(
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
        ));
        let r = ray(point(0., -1., -2.), vector(0., 0., 1.));

        let xs = t.local_intersects(t.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Arc::new(triangle(
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
        ));
        let r = ray(point(0., 0.5, -2.), vector(0., 0., 1.));

        let xs = t.local_intersects(t.clone(), r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.);
    }
}
