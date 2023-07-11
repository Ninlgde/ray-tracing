import math

import taichi as ti
from taichi.math import *

from ray import Ray
from rtweekend import random_in_unit_disk


@ti.data_oriented
class Camera:
    origin: vec3
    horizontal: vec3
    vertical: vec3
    lower_left_corner: vec3
    len_radius: float
    w: vec3
    u: vec3
    v: vec3

    def __init__(self,
                 lookfrom: vec3,
                 lookat: vec3,
                 vup: vec3,
                 vfov: float,
                 aspect_ratio: float,
                 aperture: float,
                 focus_dist: float):
        theta = vfov * math.pi / 180
        h = ti.tan(theta / 2)
        viewport_height = 2.0 * h
        viewport_width = aspect_ratio * viewport_height

        self.w = normalize(lookfrom - lookat)
        self.u = normalize(cross(vup, self.w))
        self.v = cross(self.w, self.u)

        self.origin = lookfrom
        self.horizontal = focus_dist * viewport_width * self.u
        self.vertical = focus_dist * viewport_height * self.v
        self.lower_left_corner = vec3(
            self.origin - self.horizontal / 2 - self.vertical / 2 - focus_dist * self.w)
        self.len_radius = aperture / 2

    @ti.func
    def get_ray(self, u: float, v: float) -> Ray:
        rd = self.len_radius * random_in_unit_disk()
        offset = rd[0] * self.u + rd[1] * self.v
        return Ray(self.origin + offset,
                   self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
