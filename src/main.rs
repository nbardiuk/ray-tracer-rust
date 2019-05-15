mod camera;
mod canvas;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod ppm;
mod rays;
mod spheres;
mod transformations;
mod tuples;
mod world;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use canvas::canvas;
use intersections::hit;
use intersections::Object;
use lights::point_light;
use materials::material;
use rays::ray;
use spheres::sphere;
use std::fs;
use transformations::*;
use tuples::{color, point};

fn main() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 200;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;

    let mut sphere = sphere();
    sphere.transform = shearing(1., 0., 0., 0., 0., 0.) * scaling(0.5, 1., 1.);
    sphere.material = material();
    sphere.material.color = color(0., 1., 1.);

    let ray_origin = point(0., 0., -5.);

    let light_position = point(-10., 10., -10.);
    let ligth_color = color(1., 1., 1.);
    let light = point_light(light_position, ligth_color);

    let mut canvas = canvas(canvas_pixels, canvas_pixels);
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = half - pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let ray = ray(ray_origin.clone(), (&position - &ray_origin).normalized());
            let intersects = sphere.intersects(&ray);
            let hit = hit(&intersects);
            if hit.is_some() {
                let point = ray.position(hit.unwrap().t);
                let sphere = hit.unwrap().object;
                let normal = sphere.normal_at(&point);
                let eye = -ray.direction;
                let color = sphere.material.lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
