import random

import taichi as ti
from taichi.math import *

from ray import Ray
from material import create_lambertian, create_metal, create_dielectric
from hittable_list import HittableList
from sphere import Sphere
from camera import Camera

ti.init(arch=ti.gpu)

# Image
aspect_ratio = 3.0 / 2.0
image_width = 1200
image_height = int(image_width / aspect_ratio)
samples_per_pixel = 50
max_depth = 20


# World
def random_world() -> HittableList:
    world = HittableList()
    ground_material = create_lambertian(vec3(0.5, 0.5, 0.5))
    world.append(Sphere(vec3(0, -1000, 0), 1000, ground_material))
    for a in range(-11, 11):
        for b in range(-11, 11):
            choose_mat = random.random()
            center = vec3(a + 0.9 * random.random(), 0.2, b + 0.9 * random.random())
            if (center - vec3(4, 0.2, 0)).norm() > 0.9:
                if choose_mat < 0.8:
                    # diffuse
                    albedo = vec3(random.random() * random.random(), random.random() * random.random(),
                                  random.random() * random.random())
                    sphere_material = create_lambertian(albedo)
                    world.append(Sphere(center, 0.2, sphere_material))
                elif choose_mat < 0.95:
                    # metal
                    albedo = vec3(0.5 * (1 + random.random()), 0.5 * (1 + random.random()), 0.5 * (1 + random.random()))
                    fuzz = 0.5 * random.random()
                    sphere_material = create_metal(albedo, fuzz)
                    world.append(Sphere(center, 0.2, sphere_material))
                else:
                    # glass
                    sphere_material = create_dielectric(1.5)
                    world.append(Sphere(center, 0.2, sphere_material))
    material1 = create_dielectric(1.5)
    world.append(Sphere(vec3(0, 1, 0), 1.0, material1))
    material2 = create_lambertian(vec3(0.4, 0.2, 0.1))
    world.append(Sphere(vec3(-4, 1, 0), 1.0, material2))
    material3 = create_metal(vec3(0.7, 0.6, 0.5), 0.0)
    world.append(Sphere(vec3(4, 1, 0), 1.0, material3))
    return world


world = random_world()
# world = HittableList()
#
# material_ground = create_lambertian(vec3(0.8, 0.8, 0.0))
# material_center = create_lambertian(vec3(0.1, 0.2, 0.5))
# material_left = create_dielectric(1.5)
# material_right = create_metal(vec3(0.8, 0.6, 0.2), 1.0)
#
# world.append(Sphere(vec3(0.0, -100.5, -1.0), 100, material_ground))
# world.append(Sphere(vec3(0.0, 0.0, -1.0), 0.5, material_center))
# world.append(Sphere(vec3(-1.0, 0.0, -1.0), 0.5, material_left))
# world.append(Sphere(vec3(-1.0, 0.0, -1.0), -0.4, material_left))
# world.append(Sphere(vec3(1.0, 0.0, -1.0), 0.5, material_right))
# R = math.cos(math.pi / 4)
# m1 = create_lambertian(vec3(0, 0, 1))
# m2 = create_lambertian(vec3(1, 0, 0))
# world.append(Sphere(vec3(-R, 0, -1), R, m1))
# world.append(Sphere(vec3(R, 0, -1), R, m2))

# Camera
lookfrom = vec3(13, 2, 3)
lookat = vec3(0, 0, 0)
vup = vec3(0, 1, 0)
dist_to_focus = 10.0
aperture = 0.1

pixels = ti.Vector.field(3, dtype=float, shape=(image_width, image_height))


@ti.kernel
def ti_main():
    camera = Camera(lookfrom, lookat, vup, 20, aspect_ratio, aperture, dist_to_focus)
    for i, j in pixels:
        for _ in range(samples_per_pixel):
            u = (i + ti.random()) / (image_width - 1)
            v = (j + ti.random()) / (image_height - 1)
            ray = camera.get_ray(u, v)
            pixels[i, j] += ray_color(ray, world, max_depth)
        pixels[i, j] = pixel_color(pixels[i, j], samples_per_pixel)


@ti.func
def pixel_color(color: vec3, samples_per_pixel: int):
    scale = 1.0 / samples_per_pixel
    color = ti.sqrt(scale * color)
    return color


@ti.func
def ray_color(ray: Ray, world: ti.template(), depth: int) -> vec3:
    # 需要先初始化一个变量在这里,否则会报non-static的报错
    # 详情请看: https://docs.taichi-lang.org/docs/differences_between_taichi_and_python_programs
    color = vec3(0.0)
    attenuation = vec3(1.0)
    while depth > 0:
        hit, rec = world.hit(ray, 0.0001, float('inf'))
        if hit:
            r, a, s = rec.mat.scatter(ray, rec)
            if r:
                attenuation *= a
                ray = s
            else:
                break
        else:
            unit_direction = normalize(ray.direction)
            t = 0.5 * (unit_direction[1] + 1.0)
            color = (1.0 - t) * vec3(1.0) + t * vec3(0.5, 0.7, 1.0)
            break
        depth -= 1
    color = attenuation * color
    return color


@ti.func
def hit_sphere(center: vec3, radius: float, ray: Ray) -> ti.float32:
    oc = ray.origin - center
    a = dot(ray.direction, ray.direction)
    half_b = dot(oc, ray.direction)
    c = dot(oc, oc) - radius * radius
    discriminant = half_b * half_b - a * c
    r = -1.0
    if discriminant > 0:
        r = (-half_b - ti.sqrt(discriminant)) / a
    return r


if __name__ == '__main__':
    world.make_tj()
    ti_main()
    filename = f'imwrite_export1.png'
    ti.tools.imwrite(pixels.to_numpy(), filename)
