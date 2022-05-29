use crate::{hit::Hit, matrix::Matrix4x4, ray::Ray, tuple::Tuple};

#[derive(Debug, PartialEq)]
pub enum Shapes {
    Sphere { transform: Matrix4x4 },
}

impl Shapes {
    pub fn sphere(transform: Matrix4x4) -> Self {
        Self::Sphere { transform: transform }
    }
    pub fn intersect(&self, ray: Ray) -> Option<Hit> {
        match self {
            Shapes::Sphere { transform } => {
                let r = ray.transform(*transform);
                let sphere_to_ray = r.origin - Tuple::point(0., 0., 0.);
                let a = Tuple::dot(r.direction, r.direction);
                let b = 2. * Tuple::dot(r.direction, sphere_to_ray);
                let c = Tuple::dot(sphere_to_ray, sphere_to_ray) - 1.;
                let d = (b * b) - (4. * a * c);

                if d >= 0. {
                    let t1 = (-b - d.sqrt()) / (2. * a);
                    let t2 = (-b + d.sqrt()) / (2. * a);

                    Some(Hit::new(self, vec![t1, t2]))
                } else {
                    None
                }
            }
        }
    }
}
