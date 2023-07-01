use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::{Material, Point3};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Rc<RefCell<dyn Material>>>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Option<Rc<RefCell<dyn Material>>>) -> Self
    where
        Self: Sized,
    {
        Sphere {
            center: center.clone(),
            radius,
            mat_ptr: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
