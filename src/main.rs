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

extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use camera::camera;
use canvas::canvas;
use cubes::cube;
use lights::point_light;
use planes::plane;
use std::f64::consts::PI;
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
    let (width, height) = (500, 400);

    thread::spawn(move || {
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

        let mut camera = camera(width, height, PI / 3.);
        camera.transform = view_transform(
            &point(0., 1.5, -5.),
            &point(0., 1., 0.),
            &vector(0., 1., 0.),
        );

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
