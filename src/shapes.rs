use intersections::Intersection;
use materials::Material;
use matrices::Matrix;
use rays::Ray;
use std::rc::Rc;
use tuples::Tuple;

pub trait Shape {
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);

    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);

    fn local_normal_at(&self, local_point: Tuple) -> Tuple;
    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = &self.transform().inverse() * world_point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.w = 0.;
        world_normal.normalized()
    }

    fn local_intersects(&self, rc: Rc<Shape>, local_ray: Ray) -> Vec<Intersection>;
    fn intersects(&self, rc: Rc<Shape>, inray: &Ray) -> Vec<Intersection> {
        let local_ray = inray.transform(self.transform().inverse());
        self.local_intersects(rc, local_ray)
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Shape ({:?}, {:?})", self.material(), self.transform())
    }
}

impl PartialEq<Shape> for Shape {
    fn eq(&self, shape: &Shape) -> bool {
        self.material().eq(shape.material()) && self.transform().eq(shape.transform())
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use materials::material;
    use materials::Material;
    use matrices::identity_matrix;
    use matrices::Matrix;
    use rays::Ray;
    use std::f64::consts::PI;
    use transformations::rotation_z;
    use transformations::scaling;
    use transformations::translation;
    use tuples::point;
    use tuples::vector;

    #[derive(Debug, PartialEq)]
    struct TestShape {
        transform: Matrix,
        material: Material,
    }
    impl Shape for TestShape {
        fn material(&self) -> &Material {
            &self.material
        }
        fn set_material(&mut self, material: Material) {
            self.material = material;
        }
        fn transform(&self) -> &Matrix {
            &self.transform
        }
        fn set_transform(&mut self, transform: Matrix) {
            self.transform = transform;
        }
        fn local_intersects(&self, _rc: Rc<Shape>, _local_ray: Ray) -> Vec<Intersection> {
            vec![]
        }
        fn local_normal_at(&self, local_point: Tuple) -> Tuple {
            vector(local_point.x, local_point.y, local_point.z)
        }
    }
    fn test_shape() -> TestShape {
        TestShape {
            transform: identity_matrix(),
            material: material(),
        }
    }

    #[test]
    fn the_default_transformation() {
        let s = test_shape();

        assert_eq!(s.transform(), &identity_matrix());
    }

    #[test]
    fn assiging_a_transformation() {
        let mut s = test_shape();

        s.set_transform(translation(2., 3., 4.));

        assert_eq!(s.transform(), &translation(2., 3., 4.));
    }

    #[test]
    fn the_default_material() {
        let s = test_shape();
        let m = s.material();
        assert_eq!(m, &material());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut m = material();
        m.ambient = 1.;

        let mut s = test_shape();
        s.set_material(m);

        let mut m = material();
        m.ambient = 1.;
        assert_eq!(s.material(), &m);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = test_shape();
        s.transform = translation(0., 1., 0.);

        let n = s.normal_at(&point(0., 1.70711, -0.70711));

        assert_eq!(n, vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = test_shape();
        s.transform = scaling(1., 0.5, 1.) * rotation_z(PI / 5.);

        let a = 2_f64.sqrt() / 2.;
        let n = s.normal_at(&point(0., a, -a));

        assert_eq!(n, vector(0., 0.97014, -0.24254));
    }
}
