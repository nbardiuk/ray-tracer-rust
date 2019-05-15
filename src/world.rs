use intersections::hit;
use intersections::Comps;
use intersections::Intersection;
use lights::point_light;
use lights::PointLight;
use rays::Ray;
use spheres::sphere;
use spheres::Sphere;
use transformations::scaling;
use tuples::color;
use tuples::point;
use tuples::Color;

pub struct World<T> {
    pub objects: Vec<T>,
    pub light_sources: Vec<PointLight>,
}

pub fn world() -> World<Sphere> {
    World {
        objects: vec![],
        light_sources: vec![],
    }
}
pub fn default_world() -> World<Sphere> {
    let mut s1 = sphere();
    s1.material.color = color(0.8, 1., 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere();
    s2.transform = scaling(0.5, 0.5, 0.5);
    World {
        objects: vec![s1, s2],
        light_sources: vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))],
    }
}

impl World<Sphere> {
    fn intersects<'a>(self: &'a World<Sphere>, inray: &Ray) -> Vec<Intersection<'a, Sphere>> {
        let mut xs: Vec<Intersection<'a, Sphere>> = self
            .objects
            .iter()
            .flat_map(|sphere| sphere.intersects(inray))
            .collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit<'a>(self: &World<Sphere>, comps: Comps<'a, Sphere>) -> Color {
        self.light_sources
            .iter()
            .map(|light| {
                comps
                    .object
                    .material
                    .lighting(light, &comps.point, &comps.eyev, &comps.normalv)
            })
            .fold(color(0., 0., 0.), |r, c| r + c)
    }

    pub fn color_at(self: &World<Sphere>, ray: &Ray) -> Color {
        hit(&self.intersects(ray))
            .map(|hit| self.shade_hit(hit.prepare_computations(ray)))
            .unwrap_or_else(|| color(0., 0., 0.))
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use intersections::intersection;
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

        let w = default_world();

        assert_eq!(w.light_sources, vec!(light));
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

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = &w.objects[0];
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
        let shape = &w.objects[1];
        let i = intersection(0.5, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
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
        let mut w = default_world();
        let mut outer = w.objects[0].clone();
        outer.material.ambient = 1.;
        let mut inner = w.objects[1].clone();
        inner.material.ambient = 1.;
        w.objects = vec![outer, inner];
        let r = ray(point(0., 0., 0.75), vector(0., 0., -1.));

        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material.color);
    }
}
