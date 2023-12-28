#![allow(unused_imports)]

extern crate nalgebra as na;

use crate::image::*;
use crate::object::*;
use crate::ray::*;
use crate::scene::*;

mod image;
mod object;
mod ray;
mod scene;

use anyhow::Result;

fn main() -> Result<()> {
    // let mut img = Image::new(100, 100);

    // img.save("black.png")?;

    // for i in 0..50 {
    //     for j in 0..100 {
    //         img[i][j] = Color::white();
    //     }
    // }

    // img.save("half_n_half.png")?;

    let mut s = Scene::default();

    
    let x1 = 2.;
    let y1 = 0.5;
    let z1 = -1.;
    let o = 0.5;
    
    s.add(Sphere::from(3., 0., 0., 0.5));
    s.add(Cuboid::from(x1, y1, z1, x1 + o, y1 + o, z1 + o));

    let img = s.render(1920, 1080);
    img.save("black.png")?;

    // dbg!(s, Ray::default());

    Ok(())
}
