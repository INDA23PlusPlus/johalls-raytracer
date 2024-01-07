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
        self.render_with_bounces(width, height, 0)
    }

    pub fn render_with_bounces(&self, width: usize, height: usize, bounces: usize) -> Image {
        let mut res = Image::new(width, height);
        let colors = (0..height)
            .cartesian_product(0..width)
            .collect_vec()
            .par_iter()
            .map(|(y, x)| {
                let x_pos = 0.;
                let y_pos = -(*y as f32 / height as f32 - 0.5 + 1e-6);
                let z_pos = *x as f32 / width as f32 - 0.5 + 1e-6;
                let y_pos = y_pos * height as f32 / width as f32;

                let pos = vec3f::new(x_pos, y_pos, z_pos);
                let target = pos * 4. + vec3f::new(2., 0., 0.);

                let mut ray = Ray {
                    pos,
                    dir: (target - pos).normalize(),
                };
                for _ in 0..1024 {
                    ray.step(self);
                }

                let (c, p) = ray.color(self);
                let mut color = c;
                let mut light_remaining = 1. - p;
                let p = 1. - (1. - p).powi(2);

                for _ in 0..bounces {
                    if light_remaining.abs() < 1e-2 {
                        break;
                    }
                    let normal = -ray.vec_closest(&self).unwrap().normalize();
                    let old_dir = ray.dir;

                    let new_rays = 1;

                    let mut avg_color = vec3f::zeros();
                    let mut avg_light_used = 0.;

                    for _ in 0..new_rays {
                        let old_ray = ray;
                        let mut ray = Ray {
                            pos: old_ray.pos,
                            dir: normal * 2. + old_dir,
                        };
                        ray.pos += 1e-2 * ray.dir;

                        for _ in 0..1024 {
                            ray.step(self);
                        }
                        let (c, p) = ray.color(self);

                        // let p = 1. - (1. - p).powi(2);

                        avg_color += light_remaining * c * p;
                        avg_light_used += light_remaining * p;
                    }

                    color += avg_color / new_rays as f32;
                    light_remaining -= avg_light_used / new_rays as f32;
                }

                color += light_remaining * vec3f::from(Color::white());

                color.map(|c| c.clamp(0., 1.)).into()
            })
            .collect::<Vec<_>>();

        res.pixels = colors;
        res
    }
}
