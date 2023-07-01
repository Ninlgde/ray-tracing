use ray_tracing::color::write_color;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
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

    let material_ground = Rc::new(RefCell::new(Lambertian {
        albedo: color![0.8, 0.8, 0.0],
    }));
    let material_center = Rc::new(RefCell::new(Lambertian {
        albedo: color![0.1, 0.2, 0.5],
    }));
    let material_left = Rc::new(RefCell::new(Dielectric { ir: 1.5 }));
    let material_right = Rc::new(RefCell::new(Metal {
        albedo: color![0.8, 0.6, 0.2],
        fuzz: 0.0,
    }));

    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, 0.0, -1.0],
        radius: 0.5,
        mat_ptr: Some(material_center.clone()),
    })));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, -100.5, -1.0],
        radius: 100.0,
        mat_ptr: Some(material_ground.clone()),
    })));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![-1.0, 0.0, -1.0],
        radius: 0.5,
        mat_ptr: Some(material_left.clone()),
    })));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![-1.0, 0.0, -1.0],
        radius: -0.4,
        mat_ptr: Some(material_left.clone()),
    })));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![1.0, 0.0, -1.0],
        radius: 0.5,
        mat_ptr: Some(material_right.clone()),
    })));

    // Camera
    let camera = Camera::new(
        point3![-2, 2, 1],
        point3![0, 0, -1],
        vec3![0, 1, 0],
        20.0,
        ASPECT_RATIO,
    );

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
        let mut scattered = Ray::new0();
        let mut attenuation = color![];
        if rec.mat_ptr.is_some() {
            let mat_ptr = rec.mat_ptr.as_ref().unwrap().borrow();
            if mat_ptr.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return color![];
    }
    let unit_direction = Point3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    color![1.0, 1.0, 1.0] * (1.0 - t) + color![0.5, 0.7, 1.0] * t
}
