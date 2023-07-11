import time

import taichi as ti
from taichi.math import *

from ray import Ray
from hittable import Hittable, HitRecord, Shape
from sphere import Sphere


class HittableList(Hittable):
    def __init__(self):
        super(HittableList, self).__init__(1)
        self.OS = None
        self.objs = []
        self.objs_len = 0

    def append(self, o: ti.template()):
        self.objs.append(o)

    def make_tj(self):
        self.objs_len = len(self.objs)
        OS = Sphere.field()
        ti.root.dense(ti.i, self.objs_len).place(OS)
        for i in range(OS.shape[0]):
            OS[i] = self.objs[i]
        self.OS = OS

    @ti.func
    def hit(self, ray: Ray, t_min: float, t_max: float) -> (bool, HitRecord):
        hit_anything = False
        closest_so_far = t_max
        rec = HitRecord()
        for i in range(self.objs_len):
            O = self.OS[i]
            hit, temp_rec = O.hit(ray, t_min, closest_so_far)
            if hit:
                hit_anything = True
                closest_so_far = temp_rec.t
                rec = temp_rec
        return hit_anything, rec
