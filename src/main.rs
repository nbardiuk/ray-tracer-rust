mod canvas;
mod matrices;
mod ppm;
mod transformations;
mod tuples;

use canvas::{canvas, Canvas};
use std::fs;
use tuples::{color, point, vector, Tuple};

fn main() {
    let mut projectile = projectile(point(0., 1., 0.), vector(1., 1.8, 0.).normalized() * 11.25);
    let env = environment(vector(0., -0.1, 0.), vector(-0.01, 0., 0.));
    let mut canvas = canvas(900, 550);

    loop {
        draw(&mut canvas, &projectile);
        projectile = tick(&env, projectile);
        if projectile.position.y < 0. {
            break;
        }
    }

    fs::write("./canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}

fn draw(canvas: &mut Canvas, projectile: &Projectile) {
    let x = projectile.position.x.round() as usize;
    let y = canvas.height - projectile.position.y.round() as usize;
    canvas.write_pixel(x, y, color(0., 1., 1.));
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    projectile(position, velocity)
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn projectile(position: Tuple, velocity: Tuple) -> Projectile {
    Projectile { position, velocity }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn environment(gravity: Tuple, wind: Tuple) -> Environment {
    Environment { gravity, wind }
}
