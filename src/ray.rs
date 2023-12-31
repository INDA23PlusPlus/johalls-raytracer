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
    pub fn vec_closest(&self, scene: &Scene) -> Option<vec3f> {
        scene.closest(self.pos).map(|o| {
            o.vec_to(
                self.pos - self.dir * 0.001, /* avoid clipping into objects */
            )
        })
    }

    pub fn step(&mut self, scene: &Scene) {
        let closest = scene.closest(self.pos).unwrap();
        let d = closest.dist(self.pos).clamp(0., 1.);
        self.pos += self.dir * d;
    }

    pub fn color(&self, scene: &Scene) -> (vec3f, f32) {
        // return Color {
        //     r: 1. / (1. + scene.objects.iter().next().unwrap().dist(self.pos).exp()),
        //     g: 0.,
        //     b: 0.,
        // };
        let closest = scene.closest(self.pos).unwrap();

        let d = closest.dist(self.pos);
        if d < 1e-3 {
            let dot = closest
                .vec_to(
                    self.pos - self.dir * 0.001, /* avoid clipping into objects */
                )
                .normalize()
                .dot(&self.dir)
                .abs()
                .clamp(0., 1.);
            match closest {
                Object::Cuboid(_) => (vec3f::new(1., 0., 0.), dot),
                Object::Sphere(_) => (vec3f::new(0., 1., 0.), dot),
            }
        } else {
            (vec3f::new(1., 1., 1.), 1.)
        }
    }
}
