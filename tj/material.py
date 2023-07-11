from enum import IntEnum

import taichi as ti
from taichi.math import *

from ray import Ray
from rtweekend import random_unit_vector, near_zero, random_in_unit_sphere, refract_


class MATERIAL(IntEnum):
    NONE = 0
    LAMBERTIAN = 1
    METAL = 2
    DIELECTRIC = 3


@ti.dataclass
class Material:
    typ: int = MATERIAL.NONE
    albedo: vec3 = vec3(0)
    fuzz: float = 0  # METAL: fuzz DIELECTRIC: ir

    @ti.func
    def scatter(self, ray_in: Ray, rec) -> (bool, vec3, Ray):
        r, a, s = False, vec3(0), Ray(vec3(0), vec3(0))
        if self.typ == MATERIAL.LAMBERTIAN:
            r, a, s = lambertian_scatter(ray_in, rec, self.albedo)
        elif self.typ == MATERIAL.METAL:
            r, a, s = metal_scatter(ray_in, rec, self.albedo, self.fuzz)
        elif self.typ == MATERIAL.DIELECTRIC:
            r, a, s = dielectric_scatter(ray_in, rec, self.fuzz)

        return r, a, s


def create_lambertian(albedo: vec3) -> Material:
    return Material(MATERIAL.LAMBERTIAN, albedo)


@ti.func
def lambertian_scatter(ray_in: Ray, rec, albedo) -> (bool, vec3, Ray):
    scatter_direction = rec.normal + random_unit_vector()
    if near_zero(scatter_direction):
        scatter_direction = rec.normal
    scattered = Ray(rec.p, scatter_direction)
    attenuation = albedo
    return True, attenuation, scattered


def create_metal(albedo: vec3, fuzz: float) -> Material:
    fuzz = min(fuzz, 1)
    return Material(MATERIAL.METAL, albedo, fuzz)


@ti.func
def metal_scatter(ray_in: Ray, rec, albedo, fuzz) -> (bool, vec3, Ray):
    reflected = reflect(normalize(ray_in.direction), rec.normal)
    scattered = Ray(rec.p, reflected + fuzz * random_in_unit_sphere())
    attenuation = albedo
    return dot(scattered.direction, rec.normal) > 0, attenuation, scattered


def create_dielectric(ir: float) -> Material:
    return Material(MATERIAL.DIELECTRIC, fuzz=ir)


@ti.func
def dielectric_scatter(ray_in: Ray, rec, ir) -> (bool, vec3, Ray):
    attenuation = vec3(1.0)
    refraction_ratio = ir if not rec.front_face else 1 / ir

    unit_direction = normalize(ray_in.direction)
    cos_theta = min(dot(-unit_direction, rec.normal), 1.0)
    sin_theta = ti.sqrt(1.0 - cos_theta * cos_theta)

    cannot_refract = refraction_ratio * sin_theta > 1.0
    direction = vec3(0)
    if cannot_refract or reflectance(cos_theta, refraction_ratio) > ti.random():
        direction = reflect(unit_direction, rec.normal)
    else:
        direction = refract_(unit_direction, rec.normal, refraction_ratio)

    scattered = Ray(rec.p, direction)
    return True, attenuation, scattered


@ti.func
def reflectance(cosine: float, ref_idx: float) -> float:
    r0 = (1 - ref_idx) / (1 + ref_idx)
    r0 = r0 * r0
    return r0 + (1 - r0) * ti.pow((1 - cosine), 5)
