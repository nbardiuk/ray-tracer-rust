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
    objects: Vec<T>,
    light_sources: Vec<PointLight>,
}

pub fn world() -> World<Sphere> {
    World {
        objects: vec![],
        light_sources: vec![],
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
        light_sources: vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))],
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

    pub fn shade_hit<'a>(self: &World<Sphere>, comps: Comps<'a, Sphere>) -> Color {
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
}

#[cfg(test)]
mod spec {
    use super::*;
    use intersections::intersection;
    use intersections::prepare_computations;
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

        let comps = prepare_computations(&i, &r);
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

        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }
}
