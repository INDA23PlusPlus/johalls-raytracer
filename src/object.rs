extern crate nalgebra as na;

use std::ops::Sub;

use crate::ray::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Cuboid {
    pub m: vec3f,
    pub r: vec3f,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Sphere {
    pub m: vec3f,
    pub r: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Cuboid(Cuboid),
    Sphere(Sphere),
}

impl Object {
    pub fn dist(&self, p: vec3f) -> f32 {
        match self {
            Object::Cuboid(c) => c.dist(p),
            Object::Sphere(c) => c.dist(p),
        }
    }

    pub fn vec_to(&self, p: vec3f) -> vec3f {
        match self {
            Object::Cuboid(c) => c.vec_to(p),
            Object::Sphere(c) => c.vec_to(p),
        }
    }
}

impl Sphere {
    pub fn from(x: f32, y: f32, z: f32, r: f32) -> Self {
        Sphere {
            m: vec3f::new(x, y, z),
            r,
        }
    }

    pub fn dist(&self, p: vec3f) -> f32 {
        let dist_from_center = (self.m - p).norm();

        (dist_from_center - self.r).max(0.)
    }

    pub fn vec_to(&self, p: vec3f) -> vec3f {
        let is_inside = self.dist(p) == 0.;

        if is_inside {
            (self.m - p) * 0.0001
        } else {
            self.m - p
        }
    }
}

impl Cuboid {
    pub fn from(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Cuboid {
        let bl = vec3f::new(x1, y1, z1);
        let tr = vec3f::new(x2, y2, z2);

        Cuboid {
            m: (bl + tr) / 2.,
            r: (tr - bl) / 2.,
        }
    }

    pub fn dist(&self, p: vec3f) -> f32 {
        self.vec_to(p).norm()
    }

    pub fn vec_to(&self, p: vec3f) -> vec3f {
        p.sub(self.m).abs().sub(self.r).map(|c| c.max(0.))
    }
}

#[cfg(test)]
mod dist_tests {
    use na::ComplexField;

    use super::*;

    fn close_enough(a: f32, b: f32) -> bool {
        dbg!(a, b);
        (a - b).abs() <= 1e-6 * (a.abs() + b.abs())
    }

    #[test]
    fn inside_sphere() {
        let s = Sphere {
            m: vec3f::zeros(),
            r: 1.,
        };

        assert!(close_enough(s.dist(vec3f::zeros()), 0.));
    }

    #[test]
    fn outside_sphere() {
        let s = Sphere {
            m: vec3f::zeros(),
            r: 2.,
        };

        assert_eq!(s.dist(vec3f::new(3., 0., 0.)), 1.);
    }

    #[test]
    fn inside_cuboid() {
        let c = Cuboid {
            m: vec3f::zeros(),
            r: vec3f::from_element(1.),
        };

        assert!(close_enough(c.dist(vec3f::zeros()), 0.));
        assert!(close_enough(c.dist(vec3f::new(0.5, 0.5, 0.5)), 0.));
    }

    #[test]
    fn outside_cuboid() {
        let c = Cuboid {
            m: vec3f::zeros(),
            r: vec3f::from_element(2.),
        };

        assert!(close_enough(c.dist(vec3f::new(4., 0., 0.)), 2.));
        assert!(close_enough(c.dist(vec3f::new(-5., 6., 0.)), 5.));
        assert!(close_enough(c.dist(vec3f::new(4., -4., 4.)), 12f32.sqrt()));
    }
}

pub trait IntoObject {
    fn into_object(self) -> Object;
}

impl IntoObject for Sphere {
    fn into_object(self) -> Object {
        Object::Sphere(self)
    }
}

impl IntoObject for Cuboid {
    fn into_object(self) -> Object {
        Object::Cuboid(self)
    }
}
