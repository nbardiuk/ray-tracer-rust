use intersections::Intersection;
use lights::point_light;
use lights::PointLight;
use rays::Ray;
use spheres::sphere;
use spheres::Sphere;
use transformations::scaling;
use tuples::color;
use tuples::point;

pub struct World<T> {
    objects: Vec<T>,
    light_source: Option<PointLight>,
}

pub fn world() -> World<Sphere> {
    World {
        objects: vec![],
        light_source: None,
    }
}
fn default_world() -> World<Sphere> {
    let mut s1 = sphere();
    s1.material.color = color(0.8, 1., 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere();
    s2.transform = scaling(0.5, 0.5, 0.5);
    World {
        objects: vec![s1, s2],
        light_source: Some(point_light(point(-10., 10., -10.), color(1., 1., 1.))),
    }
}

impl World<Sphere> {
    pub fn intersects<'a>(self: &'a World<Sphere>, inray: &Ray) -> Vec<Intersection<'a, Sphere>> {
        let mut xs: Vec<Intersection<'a, Sphere>> = self
            .objects
            .iter()
            .flat_map(|sphere| sphere.intersects(inray))
            .collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use lights::point_light;
    use rays::ray;
    use spheres::sphere;
    use transformations::scaling;
    use tuples::color;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn creating_a_world() {
        let w = world();
        assert_eq!(w.objects, vec!());
        assert_eq!(w.light_source, None);
    }

    #[test]
    fn the_default_world() {
        let light = point_light(point(-10., 10., -10.), color(1., 1., 1.));
        let mut s1 = sphere();
        s1.material.color = color(0.8, 1., 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.transform = scaling(0.5, 0.5, 0.5);

        let w = default_world();

        assert_eq!(w.light_source, Some(light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

        let xs = w.intersects(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }
}
