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

// a xz plane with normal pointing in the positive y direction

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub invtransform: Matrix,
    pub material: Material,
    bounds: Bounds,
}

impl Shape for Plane {
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
    fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        vector(0., 1., 0.)
    }
    fn local_intersects(&self, rc: Arc<Shape>, local_ray: Ray) -> Vec<Intersection> {
        if local_ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;
            vec![intersection(t, rc.clone())]
        }
    }
}

impl PartialEq<Plane> for Shape {
    fn eq(&self, other: &Plane) -> bool {
        self.material().eq(other.material()) && self.invtransform().eq(other.invtransform())
    }
}

pub fn plane() -> Plane {
    Plane {
        material: material(),
        invtransform: identity_matrix(),
        bounds: bound(
            point(NEG_INFINITY, NEG_INFINITY, 0.),
            point(INFINITY, INFINITY, 0.),
        ),
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = plane();

        let n1 = p.local_normal_at(point(0., 0., 0.));
        let n2 = p.local_normal_at(point(10., 0., -10.));
        let n3 = p.local_normal_at(point(-5., 0., 150.));

        assert_eq!(n1, vector(0., 1., 0.));
        assert_eq!(n2, vector(0., 1., 0.));
        assert_eq!(n3, vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Arc::new(plane());
        let r = ray(point(0., 10., 0.), vector(0., 0., 1.));

        let xs = p.local_intersects(p.clone(), r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Arc::new(plane());
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));

        let xs = p.local_intersects(p.clone(), r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Arc::new(plane());
        let r = ray(point(0., 1., 0.), vector(0., -1., 0.));

        let xs = p.local_intersects(p.clone(), r);

        assert_eq!(xs.len(), 1);
        assert_eq!(*xs[0].object, *p);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Arc::new(plane());
        let r = ray(point(0., -1., 0.), vector(0., 1., 0.));

        let xs = p.local_intersects(p.clone(), r);

        assert_eq!(xs.len(), 1);
        assert_eq!(*xs[0].object, *p);
    }

    #[test]
    fn a_bounds_of_a_plane() {
        let p = plane();

        assert_eq!(
            p.local_bounds(),
            bound(
                point(NEG_INFINITY, NEG_INFINITY, 0.),
                point(INFINITY, INFINITY, 0.)
            )
        );
    }
}
