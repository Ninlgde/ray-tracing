pub mod camera;
pub mod color;
mod macros;
pub mod ray;
pub mod rtweekend;
pub mod vec3;

pub use camera::Camera;

pub use vec3::Vec3;
pub type Color = Vec3;
pub type Point3 = Vec3;
