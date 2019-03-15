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

use canvas::canvas;
use intersections::hit;
use rays::ray;
use spheres::{intersects, sphere};
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

    let ray_origin = point(0., 0., -5.);

    let mut canvas = canvas(canvas_pixels, canvas_pixels);
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = half - pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin.clone(), (&position - &ray_origin).normalized());
            if hit(&intersects(&sphere, r)).is_some() {
                canvas.write_pixel(x, y, color(0., 1., 1.));
            }
        }
    }

    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
