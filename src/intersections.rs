use rays::Ray;
use shapes::Shape;
use std::sync::Arc;
use tuples::Tuple;

pub const EPSILON: f64 = 1e-10;

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Arc<Shape>,
}

impl PartialEq<Intersection> for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object.eq(&other.object)
    }
}

pub fn intersection(t: f64, object: Arc<Shape>) -> Intersection {
    Intersection { t, object }
}

pub fn intersections(a: Intersection, b: Intersection) -> Vec<Intersection> {
    vec![a, b]
}

pub fn hit<'a>(xs: &'a Vec<Intersection>) -> Option<&'a Intersection> {
    xs.iter()
        .filter(|x| x.t >= 0.)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

pub struct Comps {
    pub eyev: Tuple,
    pub inside: bool,
    pub normalv: Tuple,
    pub object: Arc<Shape>,
    pub over_point: Tuple,
    pub point: Tuple,
    pub under_point: Tuple,
    pub reflectv: Tuple,
    pub t: f64,
    pub n1: f64,
    pub n2: f64,
}

impl Comps {
    pub fn is_internal_reflection(&self) -> bool {
        // find the ratio of forst index of refraction to the second (Snell's Law)
        let n_ratio = self.n1 / self.n2;
        let cos_i = self.eyev.dot(&self.normalv);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        sin2_t > 1.
    }

    pub fn refracted_direction(&self) -> Tuple {
        // find the ratio of forst index of refraction to the second (Snell's Law)
        let n_ratio = self.n1 / self.n2;
        let cos_i = self.eyev.dot(&self.normalv);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        let cos_t = (1. - sin2_t).sqrt();
        &self.normalv * (n_ratio * cos_i - cos_t) - &self.eyev * n_ratio
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(&self.normalv);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1. - cos.powi(2));
            if sin2_t > 1. {
                return 1.;
            }
            let cos_t = (1. - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        return r0 + (1. - r0) * (1. - cos).powi(5);
    }
}

impl Intersection {
    pub fn prepare_computations(self: &Self, r: &Ray, xs: &[Intersection]) -> Comps {
        let mut n1 = 0.;
        let mut n2 = 0.;
        let mut containers: Vec<Arc<Shape>> = vec![];
        for x in xs {
            if self.eq(x) {
                n1 = containers
                    .last()
                    .map_or(1., |o| o.material().refractive_index);
            }
            if containers.contains(&x.object) {
                containers = containers
                    .into_iter()
                    .filter(|o| !o.eq(&x.object))
                    .collect();
            } else {
                containers.push(x.object.clone());
            }
            if self.eq(x) {
                n2 = containers
                    .last()
                    .map_or(1., |o| o.material().refractive_index);
                break;
            }
        }

        let point = r.position(self.t);
        let normalv = self.object.normal_at(&point);
        let eyev = -(&r.direction);
        let inside = normalv.dot(&eyev) < 0.;
        let normalv = if inside { -normalv } else { normalv };
        let over_point = &point + &normalv * EPSILON;
        let under_point = &point - &normalv * EPSILON;
        let reflectv = r.direction.reflect(&normalv);

        Comps {
            eyev,
            inside,
            normalv,
            object: self.object.clone(),
            over_point,
            point,
            under_point,
            reflectv,
            t: self.t,
            n1,
            n2,
        }
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use planes::plane;
    use rays::ray;
    use spheres::glass_sphere;
    use spheres::sphere;
    use transformations::scaling;
    use transformations::translation;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Arc::new(sphere());
        let i = intersection(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, *s);
    }

    #[test]
    fn aggreagating_intersections() {
        let s = Arc::new(sphere());
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s.clone());

        let xs = intersections(i1, i2);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Arc::new(sphere());
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s.clone());
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., s));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Arc::new(sphere());
        let i1 = intersection(-1., s.clone());
        let i2 = intersection(1., s.clone());
        let xs = intersections(i2, i1);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., s));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Arc::new(sphere());
        let i1 = intersection(-2., s.clone());
        let i2 = intersection(-1., s.clone());
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Arc::new(sphere());
        let i1 = intersection(5., s.clone());
        let i2 = intersection(7., s.clone());
        let i3 = intersection(-3., s.clone());
        let i4 = intersection(2., s.clone());
        let xs = &vec![i1, i2, i3, i4];

        assert_eq!(hit(&xs).unwrap(), &intersection(2., s));
    }

    #[test]
    fn precomputes_the_state_of_an_intersection() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Arc::new(sphere());
        let i = intersection(4., shape);

        let comps = i.prepare_computations(&r, &[]);

        assert_eq!(comps.t, i.t);
        assert_eq!(*comps.object, *i.object);
        assert_eq!(comps.point, point(0., 0., -1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Arc::new(sphere());
        let i = intersection(4., shape);

        let comps = i.prepare_computations(&r, &[]);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = Arc::new(sphere());
        let i = intersection(1., shape);

        let comps = i.prepare_computations(&r, &[]);

        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, point(0., 0., 1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = sphere();
        shape.invtransform = translation(0., 0., 1.).inverse();
        let i = intersection(5., Arc::new(shape));

        let comps = i.prepare_computations(&r, &[]);

        assert_that!(comps.over_point.z, lt(-EPSILON / 2.));
        assert_that!(comps.point.z, gt(comps.over_point.z));
    }

    #[test]
    fn precomputes_the_reflection_vector() {
        let sq2 = 2.0_f64.sqrt();
        let shape = plane();
        let r = ray(point(0., 1., -1.), vector(0., -sq2 / 2., sq2 / 2.));
        let i = intersection(sq2, Arc::new(shape));

        let comps = i.prepare_computations(&r, &[]);

        assert_eq!(comps.reflectv, vector(0., sq2 / 2., sq2 / 2.));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.set_invtransform(scaling(2., 2., 2.).inverse());
        a.material.refractive_index = 1.5;
        let a = Arc::new(a);
        let mut b = glass_sphere();
        b.set_invtransform(translation(0., 0., -0.25).inverse());
        b.material.refractive_index = 2.0;
        let b = Arc::new(b);
        let mut c = glass_sphere();
        c.set_invtransform(translation(0., 0., 0.25).inverse());
        c.material.refractive_index = 2.5;
        let c = Arc::new(c);
        let r = ray(point(0., 0., -4.), vector(0., 0., 1.));
        let xs = &vec![
            intersection(2., a.clone()),
            intersection(2.75, b.clone()),
            intersection(3.25, c.clone()),
            intersection(4.75, b.clone()),
            intersection(5.25, c.clone()),
            intersection(6., a.clone()),
        ];

        let comps: Vec<Comps> = xs.iter().map(|i| i.prepare_computations(&r, xs)).collect();

        assert_eq!(comps.get(0).unwrap().n1, 1.0);
        assert_eq!(comps.get(0).unwrap().n2, 1.5);
        assert_eq!(comps.get(1).unwrap().n1, 1.5);
        assert_eq!(comps.get(1).unwrap().n2, 2.0);
        assert_eq!(comps.get(2).unwrap().n1, 2.0);
        assert_eq!(comps.get(2).unwrap().n2, 2.5);
        assert_eq!(comps.get(3).unwrap().n1, 2.5);
        assert_eq!(comps.get(3).unwrap().n2, 2.5);
        assert_eq!(comps.get(4).unwrap().n1, 2.5);
        assert_eq!(comps.get(4).unwrap().n2, 1.5);
        assert_eq!(comps.get(5).unwrap().n1, 1.5);
        assert_eq!(comps.get(5).unwrap().n2, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = glass_sphere();
        shape.invtransform = translation(0., 0., 1.).inverse();
        let shape = Arc::new(shape);
        let i = intersection(5., shape.clone());
        let xs = vec![i];

        let comps = xs[0].prepare_computations(&r, &xs);

        assert_that!(comps.under_point.z, gt(EPSILON / 2.));
        assert_that!(comps.under_point.z, gt(comps.point.z));
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Arc::new(glass_sphere());
        let sq2 = 2.0_f64.sqrt();
        let r = ray(point(0., 0., sq2 / 2.), vector(0., 1., 0.));
        let xs = vec![
            intersection(-sq2 / 2., shape.clone()),
            intersection(sq2 / 2., shape.clone()),
        ];

        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert_eq!(reflectance, 1.);
    }

    #[test]
    fn the_schick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Arc::new(glass_sphere());
        let r = ray(point(0., 0., 0.), vector(0., 1., 0.));
        let xs = vec![
            intersection(-1., shape.clone()),
            intersection(1., shape.clone()),
        ];

        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert_that!(reflectance, close_to(0.04, 1e-5));
    }

    #[test]
    fn the_schick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = Arc::new(glass_sphere());
        let r = ray(point(0., 0.99, -2.), vector(0., 0., 1.));
        let xs = vec![intersection(1.8589, shape.clone())];

        let comps = xs[0].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert_that!(reflectance, close_to(0.48873, 1e-5));
    }
}
