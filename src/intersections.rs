use materials::Material;
use rays::Ray;
use tuples::Tuple;

pub trait Object: Sized {
    fn normal_at(&self, world_point: &Tuple) -> Tuple;
    fn intersects<'a>(&'a self, inray: &Ray) -> Vec<Intersection<'a, Self>>;
    fn material(&self) -> &Material;
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T: Object> {
    pub t: f64,
    pub object: &'a T,
}

pub fn intersection<'a, T: Object>(t: f64, object: &'a T) -> Intersection<'a, T> {
    Intersection { t, object }
}

pub fn intersections<'a, T: Object>(
    a: Intersection<'a, T>,
    b: Intersection<'a, T>,
) -> Vec<Intersection<'a, T>> {
    vec![a, b]
}

pub fn hit<'a, T: Object>(xs: &'a Vec<Intersection<'a, T>>) -> Option<&'a Intersection<'a, T>> {
    xs.iter()
        .filter(|x| x.t >= 0.)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

pub struct Comps<'a, T: Object> {
    pub t: f64,
    pub object: &'a T,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a, T: Object> Intersection<'a, T> {
    pub fn prepare_computations(self: &Self, r: &Ray) -> Comps<'a, T> {
        let point = r.position(self.t);
        let normalv = self.object.normal_at(&point);
        let eyev = -(&r.direction);
        let inside = normalv.dot(&eyev) < 0.;
        Comps {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv: if inside { -normalv } else { normalv },
            inside,
        }
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rays::ray;
    use spheres::sphere;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggreagating_intersections() {
        let s = sphere();
        let i1 = intersection(1., &s);
        let i2 = intersection(2., &s);

        let xs = intersections(i1, i2);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1., &s);
        let i2 = intersection(2., &s);
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., &s));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1., &s);
        let i2 = intersection(1., &s);
        let xs = intersections(i2, i1);

        assert_eq!(hit(&xs).unwrap(), &intersection(1., &s));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2., &s);
        let i2 = intersection(-1., &s);
        let xs = intersections(i1, i2);

        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = sphere();
        let i1 = intersection(5., &s);
        let i2 = intersection(7., &s);
        let i3 = intersection(-3., &s);
        let i4 = intersection(2., &s);
        let xs = &vec![i1, i2, i3, i4];

        assert_eq!(hit(&xs).unwrap(), &intersection(2., &s));
    }

    #[test]
    fn precomputes_the_state_of_an_intersection() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = sphere();
        let i = intersection(4., &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0., 0., -1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = sphere();
        let i = intersection(4., &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = sphere();
        let i = intersection(1., &shape);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, point(0., 0., 1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }
}
