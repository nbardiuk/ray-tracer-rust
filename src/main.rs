mod canvas;
mod matrices;
mod ppm;
mod rays;
mod spheres;
mod transformations;
mod tuples;

use canvas::{canvas, Canvas};
use std::f64::consts::PI;
use std::fs;
use transformations::rotation_y;
use tuples::{color, point, Tuple};

fn main() {
    let hours = (0..12)
        .map(|i| rotation_y(i as f64 * PI / 6.) * point(0., 0., 1.))
        .collect::<Vec<Tuple>>();

    let mut canvas = canvas(300, 300);
    for hour in hours {
        draw(&mut canvas, hour);
    }

    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}

fn draw(canvas: &mut Canvas, point: Tuple) {
    let half_size = (canvas.height - 1) as f64 / 2.;
    let x = (1. + point.x) * half_size;
    let y = (1. - point.z) * half_size;
    canvas.write_pixel(x.round() as usize, y.round() as usize, color(0., 1., 1.));
}
