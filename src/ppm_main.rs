use ray_tracing::color::write_color;
use ray_tracing::ray::Ray;
use ray_tracing::rtweekend::random_double;
use ray_tracing::{
    color, point3, vec3, Camera, Color, HitRecord, Hittable, HittableList, Point3, Sphere,
};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, 0.0, -1.0],
        radius: 0.5,
    })));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, -100.5, -1.0],
        radius: 100.0,
    })));

    // Camera
    let camera = Camera::new();

    let mut os = stdout();

    os.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("Error: write ppm header");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = color![];
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&mut os, &pixel_color, SAMPLES_PER_PIXEL)
                .expect("Error: write pixel color");
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return color!(0, 0, 0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&Ray::new(&rec.p, &(target - rec.p)), world, depth - 1);
    }
    let unit_direction = Point3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    color![1.0, 1.0, 1.0] * (1.0 - t) + color![0.5, 0.7, 1.0] * t
}
