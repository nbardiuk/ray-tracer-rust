use rays::Ray;
use tuples::point;

struct Sphere {}

fn sphere() -> Sphere {
    Sphere {}
}

fn intersects(sphere: Sphere, ray: Ray) -> Vec<f64> {
    let sphere_to_ray = ray.origin - point(0., 0., 0.);

    let a = ray.direction.dot(ray.direction);
    let b = 2. * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
    let discriminant = b.powi(2) - 4. * a * c;

    if discriminant < 0. {
        vec![]
    } else {
        vec![
            (-b - discriminant.sqrt()) / (2. * a),
            (-b + discriminant.sqrt()) / (2. * a),
        ]
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

        let xs = intersects(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.);
        assert_eq!(xs[1], 6.);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.);
        assert_eq!(xs[1], 5.);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(s, r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.);
        assert_eq!(xs[1], 1.);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();

        let xs = intersects(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.);
        assert_eq!(xs[1], -4.);
    }
}
