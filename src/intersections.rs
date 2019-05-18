use rays::Ray;
use shapes::Shape;
use std::rc::Rc;
use tuples::Tuple;

pub const EPSILON: f64 = 1e-10;

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<Shape>,
}

impl PartialEq<Intersection> for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object.eq(&other.object)
    }
}

pub fn intersection(t: f64, object: Rc<Shape>) -> Intersection {
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
    pub t: f64,
    pub object: Rc<Shape>,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl Intersection {
    pub fn prepare_computations(self: &Self, r: &Ray) -> Comps {
        let point = r.position(self.t);
        let normalv = self.object.normal_at(&point);
        let eyev = -(&r.direction);
        let inside = normalv.dot(&eyev) < 0.;
        let normalv = if inside { -normalv } else { normalv };
        let over_point = &point + &normalv * EPSILON;
        Comps {
            t: self.t,
            object: self.object.clone(),
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use rays::ray;
    use spheres::sphere;
    use transformations::translation;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Rc::new(sphere());
        let i = intersection(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, *s);
    }

    #[test]
    fn aggreagating_intersections() {
        let s = Rc::new(sphere());
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s.clone());

        let xs = intersections(i1, i2);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(sphere());
        let i1 = intersection(1., s.clone());
        let i2 = intersection(2., s.clone());
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., s));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(sphere());
        let i1 = intersection(-1., s.clone());
        let i2 = intersection(1., s.clone());
        let xs = intersections(i2, i1);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., s));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(sphere());
        let i1 = intersection(-2., s.clone());
        let i2 = intersection(-1., s.clone());
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Rc::new(sphere());
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
        let shape = Rc::new(sphere());
        let i = intersection(4., shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.t, i.t);
        assert_eq!(*comps.object, *i.object);
        assert_eq!(comps.point, point(0., 0., -1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Rc::new(sphere());
        let i = intersection(4., shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = Rc::new(sphere());
        let i = intersection(1., shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, point(0., 0., 1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = sphere();
        shape.transform = translation(0., 0., 1.);
        let i = intersection(5., Rc::new(shape));

        let comps = i.prepare_computations(&r);

        assert_that!(comps.over_point.z, lt(-EPSILON / 2.));
        assert_that!(comps.point.z, gt(comps.over_point.z));
    }
}
