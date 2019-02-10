#[derive(Clone)]
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
}
