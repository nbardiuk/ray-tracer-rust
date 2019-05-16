use lights::PointLight;
use tuples::{color, Color, Tuple};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub fn material() -> Material {
    Material {
        color: color(1., 1., 1.),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.,
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Tuple,
        eye: &Tuple,
        normal: &Tuple,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = &self.color * &light.intensity;

        // find the direction to the light sourse
        let lightv = (&light.position - position).normalized();

        //compute the ambient contribution
        let ambient = &effective_color * self.ambient;

        //light dot normal represents the cosine of the angle between the light vector and the
        //normal vector. A negative number means the light is on the other side of the surface.
        let light_dot_normal = lightv.dot(&normal);
        let black = color(0.0, 0.0, 0.0);
        let diffuse = if light_dot_normal < 0. || in_shadow {
            black.clone()
        } else {
            effective_color * self.diffuse * light_dot_normal
        };
        let reflectv = (-lightv).reflect(&normal);
        //relfect dot eye represents the cosine of the angle between the reflectin vector and the
        //eye vector. A negative number means the light reflects away from the eye.
        let reflect_dot_eye = reflectv.dot(&eye);
        let specular = if reflect_dot_eye <= 0. || in_shadow {
            black
        } else {
            let factor = reflect_dot_eye.powf(self.shininess);
            &light.intensity * self.specular * factor
        };

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::prelude::*;
    use lights::point_light;
    use tuples::{color, point, vector};

    #[test]
    fn the_default_material() {
        let m = material();
        assert_eq!(m.color, color(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_the_surface() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_the_surface_eye_offset_45() {
        let m = material();
        let position = point(0., 0., 0.);
        let a = 2_f64.sqrt() / 2.;
        let eyev = vector(0., a, -a);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1., 1., 1.));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_reflection_vector() {
        let m = material();
        let position = point(0., 0., 0.);
        let a = 2_f64.sqrt() / 2.;
        let eyev = vector(0., -a, -a);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., 10.), color(1., 1., 1.));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = material();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
        let in_shadow = true;

        let result = m.lighting(&light, &position, &eyev, &normalv, in_shadow);

        assert_that!(result, eq(color(0.1, 0.1, 0.1)));
    }
}
