mod camera;
pub mod color;
mod hittable;
mod hittable_list;
mod macros;
pub mod ray;
pub mod rtweekend;
mod sphere;
pub mod vec3;

pub use vec3::Vec3;
pub type Color = Vec3;
pub type Point3 = Vec3;

pub use hittable::HitRecord;
pub use hittable::Hittable;
pub use hittable_list::HittableList;

pub use sphere::Sphere;

pub use camera::Camera;
