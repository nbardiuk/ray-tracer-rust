use tuples::{Color, Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

pub fn point_light(position: Tuple, intensity: Color) -> PointLight {
    PointLight {
        intensity,
        position,
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use tuples::{color, point};

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = color(1., 1., 1.);
        let position = point(0., 0., 0.);
        let light = point_light(position.clone(), intensity.clone());
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
