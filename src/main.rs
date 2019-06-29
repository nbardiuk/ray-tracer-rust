mod bounds;
mod camera;
mod canvas;
mod cones;
mod cubes;
mod cylinders;
mod groups;
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

extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use camera::camera;
use canvas::canvas;
use cylinders::cylinder;
use cylinders::Cylinder;
use groups::group;
use groups::Group;
use lights::point_light;
use materials::material;
use materials::Material;
use patterns::checkers_pattern;
use patterns::Pattern;
use planes::plane;
use spheres::sphere;
use spheres::Sphere;
use std::f64::consts::PI;
use std::fs;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;
use transformations::*;
use tuples::{color, point, vector, Color};
use world::world;

use piston_window::*;
use sdl2_window::Sdl2Window;

fn hexagon_corner() -> Sphere {
    let mut corner = sphere();
    corner.invtransform = (translation(0., 0., -1.) * scaling(0.25, 0.25, 0.25)).inverse();
    corner
}

fn hexagon_edge() -> Cylinder {
    let mut edge = cylinder();
    edge.minimum = 0.;
    edge.maximum = 1.;
    edge.invtransform = (translation(0., 0., -1.)
        * rotation_y(-PI / 6.)
        * rotation_z(-PI / 2.)
        * scaling(0.25, 1., 0.25))
    .inverse();
    edge
}

fn hexagon_side() -> Group {
    let mut side = group();
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());
    side
}

fn hexagon() -> Group {
    let mut hex = group();

    (0..6)
        .map(|n| {
            let mut side = hexagon_side();
            side.invtransform = (rotation_y((n as f64) * PI / 3.)).inverse();
            side
        })
        .for_each(|side| {
            hex.add_child(side);
        });

    hex
}

fn main() {
    let (pixel_sender, pixel_reciever) = channel::<(usize, usize, Color)>();
    let (width, height) = (500, 500);

    thread::spawn(move || {
        let mut waffle = checkers_pattern(color(1., 0.9, 0.1), color(0.9, 1.0, 0.1));
        waffle.set_invtransform((scaling(0.03, 0.03, 0.02) * rotation_y(1.)).inverse());

        let mut floor = plane();
        floor.invtransform = scaling(10., 0.01, 10.).inverse();
        floor.material.reflective = 0.6;
        floor.material.pattern = Some(Box::new(waffle));

        let mut hex = hexagon();
        hex.invtransform =
            (translation(0., 1., 0.) * rotation_x(-PI / 8.) * rotation_z(PI / 10.)).inverse();

        let mut world = world();
        world.objects = vec![Rc::new(floor), Rc::new(hex)];
        world.light_sources = vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))];

        let mut camera = camera(width, height, PI / 3.);
        camera.invtransform =
            view_transform(&point(0., 3., -5.), &point(0., 1., 0.), &vector(0., 1., 0.)).inverse();

        camera.render_async(world, pixel_sender);
    });

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Ray Tracer", [width as u32, height as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut canvas = canvas(width, height);

    while let Some(event) = window.next() {
        // store newly rendered pixels
        while let Ok((x, y, color)) = pixel_reciever.try_recv() {
            canvas.write_pixel(x, y, color);
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::S => {
                    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
                }
                _ => {}
            };
        }

        let view = window.draw_size();
        let w = view.width / canvas.width as f64;
        let h = view.height / canvas.height as f64;
        let scale = if w < h { w } else { h };

        // display rendered pixels
        window.draw_2d(&event, |c, g, _d| {
            clear([0.8, 0.8, 0.8, 1.], g);
            for x in 0..canvas.width {
                for y in 0..canvas.height {
                    let color = canvas.pixel_at(x, y);
                    let col = [color.red as f32, color.green as f32, color.blue as f32, 1.];
                    rectangle(
                        col,
                        [x as f64 * scale, y as f64 * scale, scale, scale],
                        c.transform,
                        g,
                    );
                }
            }
        });
    }
}
