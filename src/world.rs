use intersections::hit;
use intersections::Comps;
use intersections::Intersection;
use lights::PointLight;
use rays::ray;
use rays::Ray;
use shapes::Shape;
use std::rc::Rc;
use tuples::color;
use tuples::Color;
use tuples::Tuple;

pub struct World {
    pub objects: Vec<Rc<Shape>>,
    pub light_sources: Vec<PointLight>,
}

pub fn world() -> World {
    World {
        objects: vec![],
        light_sources: vec![],
    }
}

impl World {
    fn intersects(&self, inray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersects(object.clone(), inray))
            .collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit(&self, comps: Comps) -> Color {
        self.light_sources
            .iter()
            .map(|light| {
                comps.object.material().lighting(
                    comps.object.clone(),
                    light,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(light, &comps.over_point),
                )
            })
            .fold(color(0., 0., 0.), |acc, color| acc + color)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        hit(&self.intersects(ray))
            .map(|hit| self.shade_hit(hit.prepare_computations(ray)))
            .unwrap_or_else(|| color(0., 0., 0.))
    }

    fn is_shadowed(&self, light: &PointLight, point: &Tuple) -> bool {
        let v = &light.position - point;
        let distance = v.magnitude();
        let direction = v.normalized();
        let r = ray(point.clone(), direction);
        let intersections = self.intersects(&r);
        hit(&intersections).map_or(false, |h| h.t < distance)
    }
}

#[cfg(test)]
pub mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use intersections::intersection;
    use lights::point_light;
    use rays::ray;
    use spheres::sphere;
    use transformations::scaling;
    use transformations::translation;
    use tuples::color;
    use tuples::point;
    use tuples::vector;

    pub fn default_world() -> World {
        let mut s1 = sphere();
        s1.material.color = color(0.8, 1., 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.transform = scaling(0.5, 0.5, 0.5);
        World {
            objects: vec![Rc::new(s1), Rc::new(s2)],
            light_sources: vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))],
        }
    }

    #[test]
    fn creating_a_world() {
        let w: World = world();
        assert!(w.objects.is_empty());
        assert_eq!(w.light_sources, vec!());
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
        let rc1 = Rc::new(s1);
        let rc2 = Rc::new(s2);

        let w = default_world();

        assert_eq!(w.light_sources, vec!(light));
        assert_eq!(*w.objects[0], *rc1);
        assert_eq!(*w.objects[1], *rc2);
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

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = w.objects[0].clone();
        let i = intersection(4., shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light_sources = vec![point_light(point(0., 0.25, 0.), color(1., 1., 1.))];
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = w.objects[1].clone();
        let i = intersection(0.5, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = default_world();
        w.light_sources = vec![point_light(point(0., 0., -10.), color(1., 1., 1.))];
        let s1 = sphere();
        let mut s2 = sphere();
        s2.transform = translation(0., 0., 10.);
        let s2rc = Rc::new(s2);
        w.objects.append(&mut vec![Rc::new(s1), s2rc.clone()]);
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let i = intersection(4., s2rc.clone());

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_that!(c, eq(color(0.1, 0.1, 0.1)));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 1., 0.));

        let c = w.color_at(&r);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

        let c = w.color_at(&r);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut s1 = sphere();
        s1.material.color = color(0.8, 1., 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        s1.material.ambient = 1.;
        let mut s2 = sphere();
        s2.transform = scaling(0.5, 0.5, 0.5);
        s2.material.ambient = 1.;
        let mut w = world();
        w.objects = vec![Rc::new(s1), Rc::new(s2)];
        w.light_sources = vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))];
        let r = ray(point(0., 0., 0.75), vector(0., 0., -1.));

        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0., 10., 0.);
        assert_that!(w.is_shadowed(&w.light_sources[0], &p), is(false));
    }

    #[test]
    fn the_shadow_when_nothing_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10., -10., 10.);
        assert_that!(w.is_shadowed(&w.light_sources[0], &p), is(true));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20., 20., -20.);
        assert_that!(w.is_shadowed(&w.light_sources[0], &p), is(false));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2., 2., -2.);
        assert_that!(w.is_shadowed(&w.light_sources[0], &p), is(false));
    }
}
