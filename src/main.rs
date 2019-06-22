mod camera;
mod canvas;
mod cubes;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod patterns;
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
use cubes::cube;
use lights::point_light;
use planes::plane;
use std::f64::consts::PI;
use std::fs;
use std::rc::Rc;
use transformations::*;
use tuples::{color, point, vector};
use world::world;

fn main() {
    let mut floor = plane();
    floor.transform = scaling(10., 0.01, 10.);
    floor.material.specular = 0.;
    floor.material.reflective = 1.;
    floor.material.color = color(1., 1., 1.);

    let wood = color(0.5, 0.3, 0.1);
    let leg_size = scaling(0.05, 1., 0.05);
    let table_rotation = rotation_y(PI / 6.);
    let mut top = cube();
    top.material.color = wood.clone();
    top.transform = table_rotation.clone() * scaling(1., 0.05, 1.) * translation(0., 20., 0.);
    let mut leg1 = cube();
    leg1.material.color = wood.clone();
    leg1.transform = table_rotation.clone() * translation(0.9, 0., -0.9) * leg_size.clone();
    let mut leg2 = cube();
    leg2.material.color = wood.clone();
    leg2.transform = table_rotation.clone() * translation(-0.9, 0., -0.9) * leg_size.clone();
    let mut leg3 = cube();
    leg3.material.color = wood.clone();
    leg3.transform = table_rotation.clone() * translation(0.9, 0., 0.9) * leg_size.clone();
    let mut leg4 = cube();
    leg4.material.color = wood.clone();
    leg4.transform = table_rotation.clone() * translation(-0.9, 0., 0.9) * leg_size.clone();

    let mut world = world();
    world.objects = vec![
        Rc::new(floor),
        Rc::new(top),
        Rc::new(leg1),
        Rc::new(leg2),
        Rc::new(leg3),
        Rc::new(leg4),
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
