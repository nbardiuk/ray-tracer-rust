use canvas::canvas;
use canvas::Canvas;
use matrices::identity_matrix;
use matrices::Matrix;
use rays::ray;
use rays::Ray;
use std::ops::Range;
use std::sync::mpsc::Sender;
use tuples::point;
use tuples::Color;
use world::World;
use world::MAX_REFLECTIONS;

#[derive(Clone)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    pub invtransform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

pub fn camera(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
    let half_view = (field_of_view / 2.).tan();
    let aspect = hsize as f64 / vsize as f64;
    let half_width = if aspect >= 1. {
        half_view
    } else {
        half_view * aspect
    };
    let half_height = if aspect >= 1. {
        half_view / aspect
    } else {
        half_view
    };
    Camera {
        hsize,
        vsize,
        invtransform: identity_matrix(),
        pixel_size: half_width * 2. / hsize as f64,
        half_height,
        half_width,
    }
}

impl Camera {
    fn ray_for_pixel(self: &Camera, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z = -1)
        let pixel = &self.invtransform * &point(world_x, world_y, -1.);
        let origin = &self.invtransform * &point(0., 0., 0.);
        let direction = (&pixel - &origin).normalized();

        ray(origin, direction)
    }

    pub fn render(self: &Camera, world: World) -> Canvas {
        let mut canvas = canvas(self.hsize, self.vsize);
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, MAX_REFLECTIONS);
                canvas.write_pixel(x, y, color);
            }
            eprint!(
                "rendering {} \r",
                (100.0 * x as f64) / (canvas.width as f64)
            )
        }
        canvas
    }

    pub fn render_async(
        self: &Camera,
        world: World,
        pixel_sender: Sender<(usize, usize, Color)>,
        ix: Range<usize>,
    ) -> () {
        for i in ix {
            let x = i % self.hsize;
            let y = i / self.hsize;
            let ray = self.ray_for_pixel(x, y);
            let color = world.color_at(&ray, MAX_REFLECTIONS);
            if let Err(_msg) = pixel_sender.send((x, y, color)) {
                // receiver dropped the handle
                break;
            }
        }
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use matrices::identity_matrix;
    use std::f64::consts::PI;
    use std::f64::EPSILON;
    use transformations::rotation_y;
    use transformations::translation;
    use transformations::view_transform;
    use tuples::color;
    use tuples::point;
    use tuples::vector;
    use world::spec::default_world;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = camera(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.invtransform.inverse(), identity_matrix());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = camera(200, 125, PI / 2.);
        assert_that!(c.pixel_size, close_to(0.01, EPSILON));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = camera(125, 200, PI / 2.);
        assert_that!(c.pixel_size, close_to(0.01, EPSILON));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = camera(201, 101, PI / 2.);

        let r = c.ray_for_pixel(100, 50);

        assert_that!(r.origin, eq(point(0., 0., 0.)));
        assert_that!(r.direction, eq(vector(0., 0., -1.)));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = camera(201, 101, PI / 2.);

        let r = c.ray_for_pixel(0, 0);

        assert_that!(r.origin, eq(point(0., 0., 0.)));
        assert_that!(r.direction, eq(vector(0.66519, 0.33259, -0.66851)));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = camera(201, 101, PI / 2.);
        c.invtransform = (rotation_y(PI / 4.) * translation(0., -2., 5.)).inverse();

        let r = c.ray_for_pixel(100, 50);

        let sq2 = 2.0_f64.sqrt();
        assert_that!(r.origin, eq(point(0., 2., -5.)));
        assert_that!(r.direction, eq(vector(sq2 / 2., 0., -sq2 / 2.)));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = camera(11, 11, PI / 2.);
        let from = point(0., 0., -5.);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);
        c.invtransform = view_transform(&from, &to, &up).inverse();

        let image = c.render(w);

        assert_eq!(image.pixel_at(5, 5), &color(0.38066, 0.47583, 0.2855));
    }
}
