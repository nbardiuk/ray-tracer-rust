use intersections::intersection;
use intersections::intersections;
use intersections::Intersection;
use materials::Material;
use matrices::Matrix;
use rays::Ray;
use tuples::point;
use tuples::Tuple;

pub trait Shape: Sized {
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);

    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = &self.transform().inverse() * world_point;
        let object_normal = object_point - point(0., 0., 0.);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalized()
    }

    fn intersects<'a>(&'a self, inray: &Ray) -> Vec<Intersection<'a, Self>> {
        let local_ray = inray.transform(self.transform().inverse());
        let shape_to_ray = local_ray.origin - point(0., 0., 0.);

        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2. * local_ray.direction.dot(&shape_to_ray);
        let c = shape_to_ray.dot(&shape_to_ray) - 1.;
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            intersections(
                intersection((-b - discriminant.sqrt()) / (2. * a), self),
                intersection((-b + discriminant.sqrt()) / (2. * a), self),
            )
        }
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use materials::material;
    use materials::Material;
    use matrices::identity_matrix;
    use matrices::Matrix;
    use transformations::translation;

    fn test_shape() -> impl Shape {
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
        }
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
        s.set_material(m.clone());

        assert_eq!(s.material(), &m);
    }
}
