use crate::Material;
use common::ray::Ray;
use common::{point3, vec3, Point3, Vec3};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Rc<RefCell<dyn Material>>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        HitRecord {
            p: point3![0.0, 0.0, 0.0],
            normal: vec3![0.0, 0.0, 0.0],
            mat_ptr: None,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
