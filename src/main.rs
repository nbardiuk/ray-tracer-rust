#[derive(Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}
fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}
fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use point;
    use tuple;
    use vector;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w1() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w0() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v, tuple(4.0, -4.0, 3.0, 0.0));
    }
}
