use bounds::bound_single;
use bounds::Bounds;
use intersections::Intersection;
use materials::Material;
use matrices::identity_matrix;
use matrices::Matrix;
use rays::Ray;
use shapes::Shape;
use shapes::SyncShape;
use std::sync::Arc;
use tuples::point;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Group {
    pub invtransform: Matrix,
    pub children: Vec<Arc<SyncShape>>,
    bounds: Bounds,
}

impl Group {
    pub fn add_child<T: 'static>(&mut self, child: T) -> Arc<SyncShape>
    where
        T: Shape + Sync + Send,
    {
        let c = Arc::new(child);
        self.add_child_rc(c.clone());
        c
    }
    pub fn add_child_rc(&mut self, c: Arc<SyncShape>) {
        self.children.push(c);

        let bounds: Vec<Bounds> = self
            .children
            .iter()
            .map(|c| c.local_bounds().transform(&c.invtransform().inverse()))
            .collect();
        //unsafe sum
        let mut i = bounds.into_iter();
        let first = i.next().unwrap();
        self.bounds = i.fold(first, |acc, b| acc + b);
    }
    fn wrap(&self, child: Arc<SyncShape>) -> Arc<SyncShape> {
        Arc::new(Group {
            invtransform: self.invtransform.clone(),
            children: vec![child.clone()],
            bounds: child.local_bounds(),
        })
    }
}
impl Shape for Group {
    fn local_bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn material(&self) -> &Material {
        self.children[0].material()
    }
    fn set_material(&mut self, _material: Material) {}
    fn invtransform(&self) -> &Matrix {
        &self.invtransform
    }
    fn set_invtransform(&mut self, invtransform: Matrix) {
        self.invtransform = invtransform;
    }
    fn world_to_object(&self, world_point: &Tuple) -> Tuple {
        self.children[0].world_to_object(&(self.invtransform() * world_point))
    }
    fn local_intersects(&self, _rc: Arc<SyncShape>, ray: Ray) -> Vec<Intersection> {
        if self.children.len() > 0 && self.local_bounds().intersects(&ray) {
            let mut xs: Vec<Intersection> = self
                .children
                .iter()
                .flat_map(|object| object.intersects(object.clone(), &ray))
                .map(|mut i| {
                    i.object = self.wrap(i.object);
                    i
                })
                .collect();
            xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            xs
        } else {
            vec![]
        }
    }
    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        self.children[0].local_normal_at(local_point)
    }
    fn normal_to_world(&self, local_normal: Tuple) -> Tuple {
        let mut normal =
            self.invtransform().transpose() * self.children[0].normal_to_world(local_normal);
        normal.w = 0.;
        normal.normalized()
    }
}
pub fn group() -> Group {
    Group {
        invtransform: identity_matrix(),
        children: vec![],
        bounds: bound_single(point(0., 0., 0.)),
    }
}
#[cfg(test)]
mod spec {
    use super::*;
    use bounds::bound;
    use hamcrest2::prelude::*;
    use matrices::identity_matrix;
    use rays::ray;
    use shapes::spec::test_shape;
    use spheres::sphere;
    use transformations::scaling;
    use transformations::translation;
    use tuples::point;
    use tuples::vector;

    #[test]
    fn creating_a_new_group() {
        let g = group();

        assert_eq!(g.invtransform, identity_matrix());
        assert_eq!(g.children.len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = group();
        let s = test_shape();

        let s = g.add_child(s);

        assert_eq!(g.children.len(), 1);
        assert_that!(g.children[0].clone(), eq(s));
    }

    #[test]
    fn intersecting_a_ray_with_a_empty_group() {
        let g = group();
        let g = Arc::new(g);

        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let xs = g.local_intersects(g.clone(), r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let s1 = sphere();
        let mut s2 = sphere();
        s2.invtransform = translation(0., 0., -3.).inverse();
        let mut s3 = sphere();
        s3.invtransform = translation(5., 0., 0.).inverse();
        let mut g = group();
        let s1 = g.add_child(s1);
        let s2 = g.add_child(s2);
        g.add_child(s3);
        let g = Arc::new(g);

        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = g.local_intersects(g.clone(), r);

        assert_eq!(xs.len(), 4);
        assert_that!(xs[0].object.clone(), eq(g.wrap(s2.clone())));
        assert_that!(xs[1].object.clone(), eq(g.wrap(s2.clone())));
        assert_that!(xs[2].object.clone(), eq(g.wrap(s1.clone())));
        assert_that!(xs[3].object.clone(), eq(g.wrap(s1.clone())));
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut s = sphere();
        s.invtransform = translation(5., 0., 0.).inverse();
        let mut g = group();
        g.invtransform = scaling(2., 2., 2.).inverse();
        g.add_child(s);
        let g = Arc::new(g);

        let r = ray(point(10., 0., -10.), vector(0., 0., 1.));
        let xs = g.intersects(g.clone(), &r);

        assert_eq!(xs.len(), 2);
    }
    #[test]
    fn a_bounds_of_a_group() {
        let mut s1 = sphere();
        s1.invtransform = translation(1., 1., 1.).inverse();
        let mut s2 = sphere();
        s2.invtransform = translation(-1., -2., -3.).inverse();
        let mut g = group();
        g.add_child(s1);
        g.add_child(s2);

        assert_eq!(
            g.local_bounds(),
            bound(point(-2., -3., -4.), point(2., 2., 2.))
        );
    }
}
