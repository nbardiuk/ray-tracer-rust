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
mod obj_file;
mod patterns;
mod planes;
mod ppm;
mod rays;
mod shapes;
mod spheres;
mod transformations;
mod triangles;
mod tuples;
mod world;

#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use crate::camera::camera;
use crate::canvas::canvas;
use crate::groups::Group;
use crate::lights::point_light;
use crate::obj_file::parse_obj;
use crate::patterns::checkers_pattern;
use crate::planes::plane;
use crate::transformations::*;
use crate::tuples::{color, f_u8, point, vector};
use crate::world::world;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn read_teapot() -> std::io::Result<Group> {
    let mut file = File::open("objs/teapot-low.obj")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_obj(&contents).to_group())
}

fn main() {
    let (pixel_sender, pixel_reciever) = channel::<(usize, usize, tuples::Color)>();
    let (width, height) = (2000, 2000);

    let waffle = checkers_pattern(color(1., 0.9, 0.1), color(0.9, 1.0, 0.1));

    let mut floor = plane();
    floor.invtransform = rotation_x(PI / 2.).inverse();
    floor.material.reflective = 0.6;
    floor.material.pattern = Some(Box::new(waffle));

    let teapod = read_teapot().unwrap();

    let mut world = world();
    world.objects = vec![Arc::new(floor), Arc::new(teapod)];
    world.light_sources = vec![point_light(point(30., -30., 30.), color(1., 1., 1.))];

    let mut camera = camera(width, height, PI / 3.);
    camera.invtransform = view_transform(
        &point(0., -30., 30.),
        &point(0., 1., 0.),
        &vector(0., 1., 0.),
    )
    .inverse();

    let threads = 16;
    let chunk_size = width * height / threads;
    (0..threads).for_each(|i| {
        let sender = pixel_sender.clone();
        let c = camera.clone();
        let w = world.clone();
        thread::spawn(move || {
            c.render_async(w, sender, chunk_size * i..chunk_size * (i + 1));
        });
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("render preview", width as u32, height as u32)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut view = window.into_canvas().build().unwrap();
    view.set_logical_size(width as u32, height as u32).unwrap();

    let mut canvas = canvas(width, height);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // store newly rendered pixels
        while let Ok((x, y, c)) = pixel_reciever.try_recv() {
            canvas.write_pixel(x, y, c);
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
                }
                _ => {}
            }
        }

        view.set_draw_color(Color::RGB(204, 204, 204));
        view.clear();
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                let color = canvas.pixel_at(x, y);
                view.set_draw_color(Color::RGB(
                    f_u8(color.red),
                    f_u8(color.green),
                    f_u8(color.blue),
                ));
                view.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }

        view.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
