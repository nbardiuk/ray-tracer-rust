use crate::canvas::Canvas;
use crate::tuples::f_u8;
use crate::tuples::Color;

impl Canvas {
    pub fn to_ppm(&self) -> String {
        self.ppm_header() + "\n" + &self.ppm_pixels() + "\n"
    }

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height).to_string()
    }

    fn ppm_pixels(&self) -> String {
        let rows = self.pixels.chunks(self.width);
        let lines = rows
            .map(|row| row.iter().flat_map(|pixel| colors(pixel)))
            .flat_map(|row| wrap(row, 70));
        lines.collect::<Vec<String>>().join("\n")
    }
}

fn wrap(words: impl Iterator<Item = String>, max_len: usize) -> Vec<String> {
    words.fold(vec![], |mut lines: Vec<String>, word| match lines.pop() {
        None => {
            lines.push(word);
            lines
        }
        Some(line) => {
            if line.len() + 1 + word.len() <= max_len {
                lines.push(line + " " + &word);
                lines
            } else {
                lines.push(line);
                lines.push(word);
                lines
            }
        }
    })
}

fn colors(c: &Color) -> Vec<String> {
    vec![c.red, c.green, c.blue]
        .into_iter()
        .map(|f| f_u8(f).to_string())
        .collect()
}

#[cfg(test)]
mod spec {
    use crate::canvas::canvas;
    use crate::tuples::color;

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
