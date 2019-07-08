use groups::group;
use groups::Group;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::rc::Rc;
use triangles::triangle;
use triangles::Triangle;
use tuples::point;
use tuples::Tuple;

pub struct Parsed {
    vertices: Vec<Tuple>,
    default_group: Rc<Group>,
    groups: HashMap<String, Rc<Group>>,
}

impl Parsed {
    pub fn to_group(&self) -> Group {
        let mut g = group();
        g.add_child_rc(self.default_group.clone());
        self.groups.iter().for_each(|(_k, v)| {
            g.add_child_rc(v.clone());
        });
        g
    }
    fn add_group(mut self, n: &str, g: Group) -> Parsed {
        if n == "" {
            self.default_group = Rc::new(g);
        } else {
            self.groups.insert(n.to_string(), Rc::new(g));
        }
        self
    }
}

pub fn parse_obj(text: &str) -> Parsed {
    let empty_result = Parsed {
        vertices: vec![],
        default_group: Rc::new(group()),
        groups: HashMap::default(),
    };
    let lines = text.lines().into_iter();
    let (parsed, last_name, last_group) = lines.fold(
        (empty_result, "", group()),
        |(mut parsed, mut last_name, mut last_group), line| {
            if let Ok(vertex) = parse_vertex(line) {
                parsed.vertices.push(vertex);
            } else if let Some(polygon) = parse_polygon(line) {
                for triangle in fan_triangulation(&polygon, &parsed.vertices) {
                    last_group.add_child(triangle);
                }
            } else if let Some(group_name) = parse_group(line) {
                parsed = parsed.add_group(last_name, last_group);
                last_group = group();
                last_name = group_name;
            }
            (parsed, last_name, last_group)
        },
    );
    parsed.add_group(last_name, last_group)
}

fn parse_vertex(line: &str) -> Result<Tuple, ParseFloatError> {
    let nums: Vec<&str> = line.trim_start_matches("v ").trim().split(' ').collect();
    let x = nums[0].parse::<f64>()?;
    let y = nums[1].parse::<f64>()?;
    let z = nums[2].parse::<f64>()?;
    Ok(point(x, y, z))
}

fn parse_group(line: &str) -> Option<&str> {
    if !line.starts_with("g ") {
        None
    } else {
        Some(line.trim_start_matches("g "))
    }
}

fn parse_polygon(line: &str) -> Option<Vec<usize>> {
    if !line.starts_with("f ") {
        None
    } else {
        Some(
            line.trim_start_matches("f ")
                .split(' ')
                .filter_map(|n| {
                    n.split('/').take(1).collect::<Vec<&str>>()[0].parse::<usize>().ok()
                    // n.parse::<usize>().ok()
                }
                )
                .collect(),
        )
    }
}

fn fan_triangulation(polygon: &[usize], vertices: &[Tuple]) -> Vec<Triangle> {
    let mut pairs = polygon.windows(2);
    if let &[a, _b] = pairs.next().unwrap() {
        pairs
            .map(|bc| {
                triangle(
                    vertices[a - 1].clone(),
                    vertices[bc[0] - 1].clone(),
                    vertices[bc[1] - 1].clone(),
                )
            })
            .collect()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use hamcrest2::*;
    use shapes::Shape;

    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = r#"
There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.
        "#;

        parse_obj(gibberish);

        assert!(true, "should not fail parsing");
    }

    #[test]
    fn vertex_records() {
        let file = r#"
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
        "#;

        let parsed = parse_obj(file);

        assert_eq!(parsed.vertices[0], point(-1., 1., 0.));
        assert_eq!(parsed.vertices[1], point(-1., 0.5, 0.));
        assert_eq!(parsed.vertices[2], point(1., 0., 0.));
        assert_eq!(parsed.vertices[3], point(1., 1., 0.));
    }

    #[test]
    fn parsing_triangle_faces() {
        let file = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
        "#;

        let parsed = parse_obj(file);
        let g = parsed.default_group;
        let t1 = g.children[0].clone();
        let t2 = g.children[1].clone();

        let ex1: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[1].clone(),
            parsed.vertices[2].clone(),
        ));
        let ex2: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[2].clone(),
            parsed.vertices[3].clone(),
        ));

        assert_that!(t1, eq(ex1));
        assert_that!(t2, eq(ex2));
    }
    #[test]
    fn parsing_triangle_faces_with_normals_textures() {
        let file = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1/1/1 2//2 3/3/3
f 1/2/3 3/2/1 4/2/1
        "#;

        let parsed = parse_obj(file);
        let g = parsed.default_group;
        let t1 = g.children[0].clone();
        let t2 = g.children[1].clone();

        let ex1: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[1].clone(),
            parsed.vertices[2].clone(),
        ));
        let ex2: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[2].clone(),
            parsed.vertices[3].clone(),
        ));

        assert_that!(t1, eq(ex1));
        assert_that!(t2, eq(ex2));
    }

    #[test]
    fn triangulating_polygons() {
        let file = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5
        "#;

        let parsed = parse_obj(file);
        let g = parsed.default_group;
        let t1 = g.children[0].clone();
        let t2 = g.children[1].clone();
        let t3 = g.children[2].clone();

        let ex1: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[1].clone(),
            parsed.vertices[2].clone(),
        ));
        let ex2: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[2].clone(),
            parsed.vertices[3].clone(),
        ));
        let ex3: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[3].clone(),
            parsed.vertices[4].clone(),
        ));

        assert_that!(t1, eq(ex1));
        assert_that!(t2, eq(ex2));
        assert_that!(t3, eq(ex3));
    }
    #[test]
    fn triangles_in_groups() {
        let file = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
        "#;

        let parsed = parse_obj(file);
        let g1 = parsed.groups.get("FirstGroup").unwrap();
        let t1 = g1.children[0].clone();
        let g2 = parsed.groups.get("SecondGroup").unwrap();
        let t2 = g2.children[0].clone();

        let ex1: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[1].clone(),
            parsed.vertices[2].clone(),
        ));
        let ex2: Rc<Shape> = Rc::new(triangle(
            parsed.vertices[0].clone(),
            parsed.vertices[2].clone(),
            parsed.vertices[3].clone(),
        ));

        assert_that!(t1, eq(ex1));
        assert_that!(t2, eq(ex2));
    }
    #[test]
    fn converting_an_obj_file_to_a_group() {
        let file = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
        "#;

        let parsed = parse_obj(file);
        let g1 = parsed.groups.get("FirstGroup").unwrap();
        let g2 = parsed.groups.get("SecondGroup").unwrap();
        let g = parsed.to_group();

        let ex1: Rc<Shape> = g1.clone();
        let ex2: Rc<Shape> = g2.clone();

        assert_that!(g.children[1].clone(), eq(ex1));
        assert_that!(g.children[2].clone(), eq(ex2));
    }
}
