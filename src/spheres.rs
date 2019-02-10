use intersections::{intersection, intersections, Intersection};
use rays::Ray;
use tuples::point;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {}

pub fn sphere() -> Sphere {
    Sphere {}
}

pub fn intersects<'a>(sphere: &'a Sphere, ray: Ray) -> Vec<Intersection<'a, Sphere>> {
    let sphere_to_ray = ray.origin - point(0., 0., 0.);

    let a = ray.direction.dot(ray.direction);
    let b = 2. * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
    let discriminant = b.powi(2) - 4. * a * c;

    if discriminant < 0. {
        vec![]
    } else {
        intersections(
            intersection((-b - discriminant.sqrt()) / (2. * a), sphere),
            intersection((-b + discriminant.sqrt()) / (2. * a), sphere),
        )
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use tuples::{point, vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }
}
