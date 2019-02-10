use tuples::{color, Color};

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![color(0.0, 0.0, 0.0); width * height],
    }
}

impl Canvas {
    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let i = self.width * y + x;
        self.pixels[i] = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        let i = self.width * y + x;
        self.pixels[i]
    }

    fn to_ppm(&self) -> String {
        self.ppm_header() + "\n" + &self.ppm_pixels() + "\n"
    }

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height).to_string()
    }

    fn ppm_pixels(&self) -> String {
        let rows = self.pixels.chunks(self.width);
        let rows = rows
            .map(|row| {
                row.iter()
                    .flat_map(|pixel| colors(pixel))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .map(|line| wrap(line, 70));
        rows.collect::<Vec<String>>().join("\n")
    }
}

fn wrap(line: String, max_len:usize) -> String {
    if line.len() <= max_len {
        line
    } else {
        line.split_whitespace()
            .fold(vec![], |mut acc: Vec<String>, s| match acc.pop() {
                Some(last) => {
                    if last.len() + 1 + s.len() <= max_len {
                        acc.push(last + " " + s);
                        acc
                    } else {
                        acc.push(last);
                        acc.push(s.to_string());
                        acc
                    }
                }
                None => {
                    acc.push(s.to_string());
                    acc
                }
            })
            .join("\n")
    }
}

fn colors(c: &Color) -> Vec<String> {
    vec![to255(c.red), to255(c.green), to255(c.blue)]
}

fn to255(f: f64) -> String {
    if f > 1.0 {
        to255(1.0)
    } else if f < 0.0 {
        to255(0.0)
    } else {
        (f * 255.0).ceil().to_string()
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
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let ppm = canvas(5, 3).to_ppm();
        assert_eq!(unlines(ppm.lines().take(3).collect()), "P3\n5 3\n255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        c.write_pixel(0, 0, color(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, color(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, color(-0.5, 0.0, 1.0));
        let ppm = c.to_ppm();
        let pixels = unlines(vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        ]);
        assert_eq!(unlines(ppm.lines().skip(3).take(3).collect()), pixels);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, color(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        let pixels = unlines(vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
        ]);
        assert_eq!(unlines(ppm.lines().skip(3).take(4).collect()), pixels);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline() {
        let mut ppm = canvas(5, 3).to_ppm();
        assert_eq!(ppm.pop(), Some('\n'));
    }

    fn unlines(s: Vec<&str>) -> String {
        s.join("\n")
    }
}
