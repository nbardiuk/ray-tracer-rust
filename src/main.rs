mod camera;
mod canvas;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod planes;
mod ppm;
mod rays;
mod shapes;
mod spheres;
mod transformations;
mod tuples;
mod world;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use camera::camera;
use lights::point_light;
use materials::material;
use planes::plane;
use shapes::Shape;
use spheres::sphere;
use std::f64::consts::PI;
use std::fs;
use std::rc::Rc;
use transformations::*;
use tuples::{color, point, vector};
use world::world;

fn main() {
    let mut floor = plane();
    floor.set_transform(scaling(10., 0.01, 10.));
    let mut m = material();
    m.color = color(1., 0.9, 0.9);
    m.specular = 0.;
    floor.set_material(m);

    let mut middle = sphere();
    middle.transform = translation(-0.5, 1., 0.5);
    middle.material = material();
    middle.material.color = color(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = material();
    left.material.color = color(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = world();
    world.objects = vec![
        Rc::new(floor),
        Rc::new(middle),
        Rc::new(left),
        Rc::new(right),
    ];
    world.light_sources = vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))];

    let mut camera = camera(400, 200, PI / 3.);
    camera.transform = view_transform(
        &point(0., 1.5, -5.),
        &point(0., 1., 0.),
        &vector(0., 1., 0.),
    );

    let canvas = camera.render(world);

    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
