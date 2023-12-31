use crate::*;
use anyhow::Result;
use itertools::Itertools;
use na::ComplexField;
use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn add<T: IntoObject>(&mut self, obj: T) {
        self.objects.push(obj.into_object());
    }

    pub fn closest(&self, p: vec3f) -> Option<&Object> {
        self.objects
            .iter()
            .min_by(|a, b| a.dist(p).partial_cmp(&b.dist(p)).unwrap())
    }

    pub fn render(&self, width: usize, height: usize) -> Image {
        self.render_bounces(width, height, 0)
    }

    pub fn render_bounces(&self, width: usize, height: usize, bounces: usize) -> Image {
        let mut res = Image::new(width, height);
        let colors = (0..height)
            .cartesian_product(0..width)
            .collect_vec()
            .par_iter()
            .map(|(y, x)| {
                let x_pos = 0.;
                let y_pos = -(*y as f32 / height as f32 - 0.5001);
                let z_pos = *x as f32 / width as f32 - 0.5001;
                let y_pos = y_pos * height as f32 / width as f32;

                let pos = vec3f::new(x_pos, y_pos, z_pos);
                let target = pos * 4. + vec3f::new(2., 0., 0.);

                let mut ray = Ray {
                    pos,
                    dir: (target - pos).normalize(),
                };
                for _ in 0..500 {
                    ray.step(self);
                }

                let (c, p) = ray.color(self);
                let mut color = c * p;
                let mut light_remaining = 1. - p;

                for _ in 0..bounces {
                    if light_remaining.abs() < 0.0001 {
                        break;
                    }
                    let normal = -ray.vec_closest(&self).unwrap().normalize();
                    ray = Ray {
                        pos: ray.pos,
                        dir: normal,
                    };

                    for _ in 0..500 {
                        ray.step(self);
                    }
                    let (c, p) = ray.color(self);
                    color += light_remaining * c * (1. - p);
                    light_remaining -= light_remaining * p;
                }

                color.map(|c| c.clamp(0., 1.)).into()
            })
            .collect::<Vec<_>>();

        res.pixels = colors;
        res
    }
}
