import taichi as ti
from taichi.math import *

from ray import Ray
from material import Material


@ti.dataclass
class HitRecord:
    p: vec3
    normal: vec3
    t: float
    mat: Material
    front_face: bool

    def __init__(self):
        self.p = vec3(0)
        self.normal = vec3(0)
        self.t = 0
        self.front_face = False

    @ti.func
    def set_face_normal(self, ray: Ray, outward_normal: vec3):
        self.front_face = dot(ray.direction, outward_normal) < 0
        self.normal = outward_normal if self.front_face else -outward_normal


@ti.dataclass
class Shape:
    type: int


@ti.data_oriented
class Hittable:
    shape: Shape

    def __init__(self, shape: int = 0):
        self.shape = shape

    @ti.func
    def hit(self, ray: Ray, t_min: float, t_max: float) -> (bool, HitRecord):
        pass
