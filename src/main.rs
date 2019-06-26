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
use cones::cone;
use cylinders::cylinder;
use lights::point_light;
use patterns::checkers_pattern;
use patterns::Pattern;
use planes::plane;
use spheres::sphere;
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

fn main() {
    let (pixel_sender, pixel_reciever) = channel::<(usize, usize, Color)>();
    let (width, height) = (300, 400);

    thread::spawn(move || {
        let mut floor = plane();
        floor.invtransform = scaling(10., 0.01, 10.).inverse();
        floor.material.reflective = 0.6;
        floor.material.color = color(0.5, 0.5, 0.5);

        let mut ice = sphere();
        ice.invtransform = (translation(0., 2.1, -1.) * scaling(0.91, 0.91, 0.91)).inverse();

        let mut waffle = checkers_pattern(color(1., 0.9, 0.1), color(0.8, 0.7, 0.1));
        waffle.set_invtransform(scaling(0.03, 0.02, 0.01).inverse());

        let mut cone = cone();
        cone.maximum = 1.;
        cone.minimum = 0.;
        cone.closed = true;
        cone.invtransform = (translation(0., 0., -1.) * scaling(1., 2., 1.)).inverse();
        cone.material.pattern = Some(Box::new(waffle));
        cone.material.shininess = 100.;

        let mut cup = cylinder();
        cup.maximum = 1.5;
        cup.minimum = 0.;
        cup.material.reflective = 0.5;
        cup.material.transparency = 1.;
        cup.material.refractive_index = 1.5;
        cup.material.color = color(0.2, 0.2, 0.2);
        cup.invtransform = translation(0.12, 0., -1.).inverse();

        let mut cup_bot = cylinder();
        cup_bot.maximum = 0.1;
        cup_bot.minimum = 0.;
        cup_bot.closed = true;
        cup_bot.material.reflective = 0.5;
        cup_bot.material.transparency = 1.;
        cup_bot.material.refractive_index = 1.5;
        cup_bot.material.color = color(0.2, 0.2, 0.2);
        cup_bot.invtransform = translation(0.12, 0., -1.).inverse();

        let mut world = world();
        world.objects = vec![
            Rc::new(floor),
            Rc::new(ice),
            Rc::new(cone),
            Rc::new(cup),
            Rc::new(cup_bot),
        ];
        world.light_sources = vec![point_light(point(-10., 10., -10.), color(1., 1., 1.))];

        let mut camera = camera(width, height, PI / 3.);
        camera.invtransform = view_transform(
            &point(0., 4., -10.),
            &point(0., 1., 0.),
            &vector(0., 1., 0.),
        ).inverse();

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
