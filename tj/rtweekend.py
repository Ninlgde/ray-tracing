import taichi
import taichi as ti
from taichi.math import *

infinity = float('inf')
pi = 3.1415926535897932385


@ti.func
def degrees_to_radians(degrees: float) -> float:
    return degrees * pi / 180.0


@ti.func
def random_in_unit_sphere() -> vec3:
    p = 2.0 * vec3(ti.random(), ti.random(), ti.random()) - vec3(1, 1, 1)
    while p.norm_sqr() >= 1.0:
        p = 2.0 * vec3(ti.random(), ti.random(), ti.random()) - vec3(1, 1, 1)
    return p


@ti.func
def random_unit_vector() -> vec3:
    p = random_in_unit_sphere()
    return normalize(p)


@ti.func
def random_in_hemisphere(normal: vec3) -> vec3:
    in_unit_sphere = random_in_unit_sphere()
    if dot(in_unit_sphere, normal) < 0.0:
        in_unit_sphere = -in_unit_sphere
    return in_unit_sphere


@ti.func
def random_in_unit_disk() -> vec3:
    p = 2.0 * vec3(ti.random(), ti.random(), 0) - vec3(1, 1, 0)
    while dot(p, p) >= 1.0:
        p = 2.0 * vec3(ti.random(), ti.random(), 0) - vec3(1, 1, 0)
    return p


@ti.func
def near_zero(vec: vec3) -> bool:
    s = 1e-8
    return (-s < vec[0] < s) and (-s < vec[1] < s) and (-s < vec[2] < s)


@ti.func
def refract_(x, n, eta):
    cos_theta = min(dot(-x, n), 1.0)
    r_out_perp = eta * (x + cos_theta * n)
    r_out_parallel = -sqrt(abs(1.0 - r_out_perp.norm_sqr())) * n
    return r_out_perp + r_out_parallel
