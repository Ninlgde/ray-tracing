use crate::Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new0() -> Self {
        Ray {
            origin: Point3::new0(),
            direction: Point3::new0(),
        }
    }
    pub fn new(origin: &Point3, direction: &Point3) -> Self
    where
        Self: Sized,
    {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
