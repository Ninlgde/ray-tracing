use ray_tracing::color::write_color;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::ray::Ray;
use ray_tracing::rtweekend::{random_double, random_double_range};
use ray_tracing::{
    color, point3, vec3, Camera, Color, HitRecord, Hittable, HittableList, Material, Point3, Sphere,
};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = point3![12, 2, 3];
    let lookat = point3![0, 0, -1];
    let vup = vec3![0, 1, 0];
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, 0.1, 10.0);

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(RefCell::new(Lambertian {
        albedo: color![0.5, 0.5, 0.5],
    }));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, -1000.0, 0.0],
        radius: 1000.0,
        mat_ptr: Some(ground_material.clone()),
    })));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = point3![
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double()
            ];

            if (center - point3![4.0, 0.2, 0.0]).length() > 0.9 {
                let sphere_material: Rc<RefCell<dyn Material>>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(RefCell::new(Lambertian { albedo }));
                    world.add(Rc::new(RefCell::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material.clone()),
                    })));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(RefCell::new(Metal { albedo, fuzz }));
                    world.add(Rc::new(RefCell::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material.clone()),
                    })));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(Dielectric { ir: 1.5 }));
                    world.add(Rc::new(RefCell::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material.clone()),
                    })));
                }
            }
        }
    }

    let material1 = Rc::new(RefCell::new(Dielectric { ir: 1.5 }));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![0.0, 1.0, 0.0],
        radius: 1.0,
        mat_ptr: Some(material1.clone()),
    })));

    let material2 = Rc::new(RefCell::new(Lambertian {
        albedo: color![0.4, 0.2, 0.1],
    }));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![-4.0, 1.0, 0.0],
        radius: 1.0,
        mat_ptr: Some(material2.clone()),
    })));

    let material3 = Rc::new(RefCell::new(Metal {
        albedo: color![0.7, 0.6, 0.5],
        fuzz: 0.0,
    }));
    world.add(Rc::new(RefCell::new(Sphere {
        center: point3![4.0, 1.0, 0.0],
        radius: 1.0,
        mat_ptr: Some(material3.clone()),
    })));

    world
}
