﻿use vec::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin : Vec3,
    pub direction : Vec3
}

impl Ray {
    pub fn ray(orig : Vec3, dir : Vec3) -> Ray {
        Ray {
            origin : orig,
            direction : dir
        }
    }

    pub fn at(self, t : f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

