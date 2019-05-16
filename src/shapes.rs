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

    fn transform(&self) -> &Matrix;

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = &self.transform().inverse() * world_point;
        let object_normal = object_point - point(0., 0., 0.);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalized()
    }

    fn intersects<'a>(&'a self, inray: &Ray) -> Vec<Intersection<'a, Self>> {
        let ray = inray.transform(self.transform().inverse());
        let sphere_to_ray = ray.origin - point(0., 0., 0.);

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
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
