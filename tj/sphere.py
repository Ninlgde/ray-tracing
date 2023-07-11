import taichi as ti
from taichi.math import *

from ray import Ray
from hittable import Hittable, HitRecord
from material import Material


@ti.dataclass
class Sphere(Hittable):
    center: vec3
    radius: float
    mat: Material

    def __init__(self, center: vec3, radius: float, mat: Material):
        super(Sphere, self).__init__(2)
        self.center = center
        self.radius = radius
        self.mat = mat

    @ti.func
    def hit(self, ray: Ray, t_min: float, t_max: float) -> (bool, HitRecord):
        hit = True
        oc = ray.origin - self.center
        a = dot(ray.direction, ray.direction)
        half_b = dot(oc, ray.direction)
        c = dot(oc, oc) - self.radius * self.radius
        discriminant = half_b * half_b - a * c
        if discriminant < 0:
            hit = False
        sqrtd = ti.sqrt(discriminant)
        root = (-half_b - sqrtd) / a
        if root < t_min or t_max < root:
            root = (-half_b + sqrtd) / a
            if root < t_min or t_max < root:
                hit = False
        rec = HitRecord()
        if hit:
            rec.t = root
            rec.p = ray.at(rec.t)
            outward_normal = (rec.p - self.center) / self.radius
            rec.set_face_normal(ray, outward_normal)
            rec.mat = self.mat

        return hit, rec
