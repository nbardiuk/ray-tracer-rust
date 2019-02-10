#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T> {
    pub t: f64,
    pub object: &'a T,
}

pub fn intersection<'a, T>(t: f64, object: &'a T) -> Intersection<'a, T> {
    Intersection { t, object }
}

pub fn intersections<'a, T>(
    a: Intersection<'a, T>,
    b: Intersection<'a, T>,
) -> Vec<Intersection<'a, T>> {
    vec![a, b]
}

pub fn hit<'a, T>(xs: &'a Vec<Intersection<'a, T>>) -> Option<&'a Intersection<'a, T>> {
    xs.iter()
        .filter(|x| x.t >= 0.)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod spec {
    use super::*;
    use spheres::sphere;

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
}
