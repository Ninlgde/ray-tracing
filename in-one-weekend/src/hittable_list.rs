use crate::{HitRecord, Hittable};
use common::ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object
                .borrow()
                .hit(ray, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
