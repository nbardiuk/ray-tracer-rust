use tuples::{color, Color};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![color(0.0, 0.0, 0.0); width * height],
    }
}

impl Canvas {
    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let i = self.width * y + x;
        self.pixels[i] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let i = self.width * y + x;
        self.pixels[i].clone()
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use tuples::color;

    #[test]
    fn creating_a_canvas() {
        let black = color(0.0, 0.0, 0.0);
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.iter().all(|color| color.eq(&black)));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        assert_eq!(c.pixel_at(2, 3), color(0.0, 0.0, 0.0));
        c.write_pixel(2, 3, red.clone());
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
