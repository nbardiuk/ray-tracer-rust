use crate::intersections::hit;
use crate::intersections::Comps;
use crate::intersections::Intersection;
use crate::lights::PointLight;
use crate::rays::ray;
use crate::rays::Ray;
use crate::shapes::SyncShape;
use crate::tuples::color;
use crate::tuples::Color;
use crate::tuples::Tuple;
use std::sync::Arc;

pub const MAX_REFLECTIONS: i8 = 6;

#[derive(Clone)]
pub struct World {
    pub objects: Vec<Arc<SyncShape>>,
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

    fn shade_hit(&self, comps: Comps, remaining: i8) -> Color {
        self.light_sources
            .iter()
            .map(|light| {
                let material = comps.object.material();

                let (refl, refr) = if material.reflective > 0. && material.transparency > 0. {
                    let reflectance = comps.schlick();
                    (reflectance, 1. - reflectance)
                } else {
                    (1., 1.)
                };

                return material.lighting(
                    comps.object.clone(),
                    light,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(light, &comps.over_point),
                ) + self.reflected_color(&comps, remaining) * refl
                    + self.refracted_color(&comps, remaining) * refr;
            })
            .fold(color(0., 0., 0.), |acc, color| acc + color)
    }

    pub fn color_at(&self, ray: &Ray, remaining: i8) -> Color {
        let xs = &self.intersects(ray);
        hit(xs)
            .map(|hit| self.shade_hit(hit.prepare_computations(ray, xs), remaining))
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

    fn reflected_color(&self, comps: &Comps, remaining: i8) -> Color {
        if remaining < 1 || comps.object.material().reflective == 0. {
            color(0., 0., 0.)
        } else {
            let reflect_ray = ray(comps.over_point.clone(), comps.reflectv.clone());
            self.color_at(&reflect_ray, remaining - 1) * comps.object.material().reflective
        }
    }

    fn refracted_color(&self, comps: &Comps, remaining: i8) -> Color {
        if remaining == 0 {
            return color(0., 0., 0.);
        }
        if comps.object.material().transparency == 0. {
            return color(0., 0., 0.);
        }
        if comps.is_internal_reflection() {
            return color(0., 0., 0.);
        }
        let refract_ray = ray(comps.under_point.clone(), comps.refracted_direction());
        self.color_at(&refract_ray, remaining - 1) * comps.object.material().transparency
    }
}

#[cfg(test)]
pub mod spec {
    use super::*;
    use crate::intersections::intersection;
    use crate::lights::point_light;
    use crate::patterns::spec::test_pattern;
    use crate::planes::plane;
    use crate::rays::ray;
    use crate::spheres::sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuples::color;
    use crate::tuples::point;
    use crate::tuples::vector;
    use hamcrest2::prelude::*;

    pub fn default_world() -> World {
        let mut s1 = sphere();
        s1.material.color = color(0.8, 1., 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.invtransform = scaling(0.5, 0.5, 0.5).inverse();
        World {
            objects: vec![Arc::new(s1), Arc::new(s2)],
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
        s2.invtransform = scaling(0.5, 0.5, 0.5).inverse();
        let rc1: Arc<SyncShape> = Arc::new(s1);
        let rc2: Arc<SyncShape> = Arc::new(s2);

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

        let comps = i.prepare_computations(&r, &[]);
        let c = w.shade_hit(comps, MAX_REFLECTIONS);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light_sources = vec![point_light(point(0., 0.25, 0.), color(1., 1., 1.))];
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = w.objects[1].clone();
        let i = intersection(0.5, shape);

        let comps = i.prepare_computations(&r, &[]);
        let c = w.shade_hit(comps, MAX_REFLECTIONS);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = default_world();
        w.light_sources = vec![point_light(point(0., 0., -10.), color(1., 1., 1.))];
        let s1 = sphere();
        let mut s2 = sphere();
        s2.invtransform = translation(0., 0., 10.).inverse();
        let s2rc = Arc::new(s2);
        w.objects.append(&mut vec![Arc::new(s1), s2rc.clone()]);
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let i = intersection(4., s2rc.clone());

        let comps = i.prepare_computations(&r, &[]);
        let c = w.shade_hit(comps, MAX_REFLECTIONS);

        assert_that!(c, eq(color(0.1, 0.1, 0.1)));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 1., 0.));

        let c = w.color_at(&r, MAX_REFLECTIONS);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

        let c = w.color_at(&r, MAX_REFLECTIONS);

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
        s2.invtransform = scaling(0.5, 0.5, 0.5).inverse();
        s2.material.ambient = 1.;
        let mut w = world();
        w.objects = vec![Arc::new(s1), Arc::new(s2)];
        w.light_sources = vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))];
        let r = ray(point(0., 0., 0.75), vector(0., 0., -1.));

        let c = w.color_at(&r, MAX_REFLECTIONS);

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

    #[test]
    fn the_reflected_color_for_nonreflective_material() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let mut s1 = sphere();
        s1.material.color = color(0.8, 1., 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.invtransform = scaling(0.5, 0.5, 0.5).inverse();
        s2.material.ambient = 1.;
        let shape = Arc::new(s2);
        let w = World {
            objects: vec![Arc::new(s1), shape.clone()],
            light_sources: vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))],
        };
        let i = intersection(1., shape.clone());

        let comps = i.prepare_computations(&r, &[]);
        let c = w.reflected_color(&comps, MAX_REFLECTIONS);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.invtransform = translation(0., -1., 0.).inverse();
        let s = Arc::new(shape);
        let mut w = default_world();
        w.objects.push(s.clone());
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., -3.), vector(0., -sq2 / 2., sq2 / 2.));
        let i = intersection(sq2, s.clone());

        let comps = i.prepare_computations(&r, &[]);
        let c = w.reflected_color(&comps, MAX_REFLECTIONS);

        assert_eq!(c, color(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.invtransform = translation(0., -1., 0.).inverse();
        let s = Arc::new(shape);
        let mut w = default_world();
        w.objects.push(s.clone());
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., -3.), vector(0., -sq2 / 2., sq2 / 2.));
        let i = intersection(sq2, s.clone());

        let comps = i.prepare_computations(&r, &[]);
        let c = w.shade_hit(comps, MAX_REFLECTIONS);

        assert_eq!(c, color(0.87675, 0.92433, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut lower = plane();
        lower.material.reflective = 1.;
        lower.invtransform = translation(0., -1., 0.).inverse();
        let mut upper = plane();
        upper.material.reflective = 1.;
        upper.invtransform = translation(0., 1., 0.).inverse();
        let mut w = world();
        w.light_sources = vec![point_light(point(0., 0., 0.), color(1., 1., 1.))];
        w.objects = vec![Arc::new(lower), Arc::new(upper)];
        let r = ray(point(0., 0., 0.), vector(0., 1., 0.));

        assert_eq!(w.color_at(&r, MAX_REFLECTIONS), color(13.3, 13.3, 13.3)); //exits recursion
    }

    #[test]
    fn the_reflected_color_at_maximum_recursive_depth() {
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.invtransform = translation(0., -1., 0.).inverse();
        let s = Arc::new(shape);
        let mut w = default_world();
        w.objects.push(s.clone());
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., -3.), vector(0., -sq2 / 2., sq2 / 2.));
        let i = intersection(sq2, s.clone());

        let comps = i.prepare_computations(&r, &[]);
        let c = w.reflected_color(&comps, 0);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let shape = w.objects[0].clone();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = vec![
            intersection(4., shape.clone()),
            intersection(6., shape.clone()),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut shape = sphere();
        shape.material.color = color(0.8, 1., 0.6);
        shape.material.diffuse = 0.7;
        shape.material.specular = 0.2;
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
        let shape = Arc::new(shape);
        let mut w = default_world();
        w.objects[0] = shape.clone();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = vec![
            intersection(4., shape.clone()),
            intersection(6., shape.clone()),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut shape = sphere();
        shape.material.color = color(0.8, 1., 0.6);
        shape.material.diffuse = 0.7;
        shape.material.specular = 0.2;
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
        let shape = Arc::new(shape);
        let mut w = default_world();
        w.objects[0] = shape.clone();
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., sq2 / 2.), vector(0., 1., 0.));
        let xs = vec![
            intersection(-sq2 / 2., shape.clone()),
            intersection(sq2 / 2., shape.clone()),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, color(0., 0., 0.));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut a = sphere();
        a.material.color = color(0.8, 1., 0.6);
        a.material.diffuse = 0.7;
        a.material.specular = 0.2;
        a.material.ambient = 1.;
        a.material.pattern = Some(Box::new(test_pattern()));
        let a = Arc::new(a);
        let mut b = sphere();
        b.invtransform = scaling(0.5, 0.5, 0.5).inverse();
        b.material.transparency = 1.;
        b.material.refractive_index = 1.5;
        let b = Arc::new(b);
        let mut w = default_world();
        w.objects[0] = a.clone();
        w.objects[1] = b.clone();
        let r = ray(point(0., 0., 0.1), vector(0., 1., 0.));
        let xs = vec![
            intersection(-0.9899, a.clone()),
            intersection(-0.4899, b.clone()),
            intersection(0.4899, b.clone()),
            intersection(0.9899, a.clone()),
        ];

        let comps = xs[2].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, color(0., 0.99888, 0.04722));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut floor = plane();
        floor.invtransform = translation(0., -1., 0.).inverse();
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        let floor = Arc::new(floor);
        let mut ball = sphere();
        ball.material.color = color(1., 0., 0.);
        ball.material.ambient = 0.5;
        ball.invtransform = translation(0., -3.5, -0.5).inverse();
        let ball = Arc::new(ball);
        let mut w = default_world();
        w.objects.push(floor.clone());
        w.objects.push(ball.clone());
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., -3.), vector(0., -sq2 / 2., sq2 / 2.));
        let xs = vec![intersection(sq2, floor.clone())];

        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut floor = plane();
        floor.invtransform = translation(0., -1., 0.).inverse();
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        let floor = Arc::new(floor);
        let mut ball = sphere();
        ball.material.color = color(1., 0., 0.);
        ball.material.ambient = 0.5;
        ball.invtransform = translation(0., -3.5, -0.5).inverse();
        let ball = Arc::new(ball);
        let mut w = default_world();
        w.objects.push(floor.clone());
        w.objects.push(ball.clone());
        let sq2 = 2_f64.sqrt();
        let r = ray(point(0., 0., -3.), vector(0., -sq2 / 2., sq2 / 2.));
        let xs = vec![intersection(sq2, floor.clone())];

        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
