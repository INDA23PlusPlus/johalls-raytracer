extern crate nalgebra as na;

#[allow(non_camel_case_types)]
pub type vec3f = na::Vector3<f32>;

use std::ops::Sub;

use crate::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub pos: vec3f,
    pub dir: vec3f,
}

impl Ray {
    pub fn step(&mut self, scene: &Scene) {
        let closest = scene.closest(self.pos).unwrap();
        let d = closest.dist(self.pos);
        self.pos += self.dir * d;
    }

    pub fn color(&self, scene: &Scene) -> Color {
        let closest = scene.closest(self.pos).unwrap();

        let d = closest.dist(self.pos);
        return if d < 1e-5 {
            let dot = closest.vec_to(self.pos - self.dir * 0.001).normalize().dot(&self.dir).abs();
            match closest {
                Object::Cuboid(_) => Color {
                    r: dot,
                    g: 0.,
                    b: 0.,
                },
                Object::Sphere(_) => Color {
                    r: 0.,
                    g: dot,
                    b: 0.,
                },
            }
        } else {
            Color::white()
        };
    }
}
