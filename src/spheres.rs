use intersections::intersection;
use intersections::intersections;
use intersections::Intersection;
use materials::{material, Material};
use matrices::{identity_matrix, Matrix};
use rays::Ray;
use shapes::Shape;
use std::rc::Rc;
use tuples::point;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub invtransform: Matrix,
    pub material: Material,
}

pub fn sphere() -> Sphere {
    Sphere {
        invtransform: identity_matrix(),
        material: material(),
    }
}

pub fn glass_sphere() -> Sphere {
    let mut sphere = sphere();
    sphere.material.transparency = 1.0;
    sphere.material.refractive_index = 1.5;
    sphere
}

impl Shape for Sphere {
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
    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - point(0., 0., 0.)
    }
    fn local_intersects(&self, rc: Rc<Shape>, local_ray: Ray) -> Vec<Intersection> {
        let shape_to_ray = local_ray.origin - point(0., 0., 0.);

        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2. * local_ray.direction.dot(&shape_to_ray);
        let c = shape_to_ray.dot(&shape_to_ray) - 1.;
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            intersections(
                intersection((-b - discriminant.sqrt()) / (2. * a), rc.clone()),
                intersection((-b + discriminant.sqrt()) / (2. * a), rc.clone()),
            )
        }
    }
}

impl PartialEq<Sphere> for Shape {
    fn eq(&self, other: &Sphere) -> bool {
        self.material().eq(other.material()) && self.invtransform().eq(other.invtransform())
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use transformations::{scaling, translation};
    use tuples::{point, vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = Rc::new(sphere());

        let xs = s.local_intersects(s.clone(), r);

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].object, *s);
        assert_eq!(*xs[1].object, *s);
    }

    #[test]
    fn intersection_a_scaled_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.invtransform = scaling(2., 2., 2.).inverse();
        let rc = Rc::new(s);

        let xs = rc.intersects(rc.clone(), &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersection_a_translated_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.invtransform = translation(5., 0., 0.).inverse();
        let rc = Rc::new(s);

        let xs = rc.intersects(rc.clone(), &r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = s.local_normal_at(point(1., 0., 0.));
        assert_eq!(n, vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = s.local_normal_at(point(0., 1., 0.));
        assert_eq!(n, vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = s.local_normal_at(point(0., 0., 1.));
        assert_eq!(n, vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = sphere();
        let a = 3_f64.sqrt() / 3.;
        let n = s.local_normal_at(point(a, a, a));
        assert_eq!(n, vector(a, a, a));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = sphere();
        let a = 3_f64.sqrt() / 3.;
        let n = s.local_normal_at(point(a, a, a));
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();

        assert_eq!(s.invtransform, identity_matrix());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
