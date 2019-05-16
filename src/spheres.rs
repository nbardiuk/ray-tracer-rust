use materials::{material, Material};
use matrices::{identity_matrix, Matrix};
use shapes::Shape;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

pub fn sphere() -> Sphere {
    Sphere {
        transform: identity_matrix(),
        material: material(),
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    fn transform(&self) -> &Matrix {
        &self.transform
    }
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use matrices::identity_matrix;
    use rays::ray;
    use std::f64::consts::PI;
    use transformations::{rotation_z, scaling, translation};
    use tuples::{point, vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = sphere();
        assert_eq!(s.transform, identity_matrix());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = sphere();
        s.transform = translation(2., 3., 4.);
        assert_eq!(s.transform, translation(2., 3., 4.));
    }

    #[test]
    fn intersection_a_scaled_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.transform = scaling(2., 2., 2.);

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersection_a_translated_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.transform = translation(5., 0., 0.);

        let xs = s.intersects(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = s.normal_at(&point(1., 0., 0.));
        assert_eq!(n, vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0., 1., 0.));
        assert_eq!(n, vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0., 0., 1.));
        assert_eq!(n, vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = sphere();
        let a = 3_f64.sqrt() / 3.;
        let n = s.normal_at(&point(a, a, a));
        assert_eq!(n, vector(a, a, a));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = sphere();
        let a = 3_f64.sqrt() / 3.;
        let n = s.normal_at(&point(a, a, a));
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = sphere();
        s.transform = translation(0., 1., 0.);

        let n = s.normal_at(&point(0., 1.70711, -0.70711));

        assert_eq!(n, vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = sphere();
        s.transform = scaling(1., 0.5, 1.) * rotation_z(PI / 5.);

        let a = 2_f64.sqrt() / 2.;
        let n = s.normal_at(&point(0., a, -a));

        assert_eq!(n, vector(0., 0.97014, -0.24254));
    }
}
