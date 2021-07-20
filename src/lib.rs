#[macro_use]
extern crate serde_derive;

pub mod color;
pub mod element;
pub mod light;
pub mod material;
pub mod point;
mod rendering;
pub mod scene;
pub mod vector;

use std::sync::Arc;

use color::Color;
use rayon::prelude::*;
use rendering::{cast_ray, Ray};

#[derive(Debug)]
struct RayTask {
    x: u32,
    y: u32,
    color: Color,
}

pub fn render(scene: scene::Scene, buffer: &mut [u8], bytes_per_pixel: u8) {
    if bytes_per_pixel != 3 && bytes_per_pixel != 4 {
        // TODO: Maybe return a result?
        return;
    }

    let scene = Arc::new(scene);
    let write_pixel = match bytes_per_pixel {
        4 => crate::write_rgba_pixel,
        3 => crate::write_rgb_pixel,
        _ => crate::write_dummy_pixel,
    };

    buffer
        .par_chunks_exact_mut(bytes_per_pixel as usize)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % scene.width as usize;
            let y = i / scene.width as usize;

            let ray = Ray::create_prime(x as u64, y as u64, scene.clone());
            let color = cast_ray(scene.clone(), &ray, 0);
            write_pixel(color, pixel);
        });
}

fn write_rgba_pixel(color: Color, pixel: &mut [u8]) {
    let rgba = color.to_rgba();
    pixel[0] = rgba[0];
    pixel[1] = rgba[1];
    pixel[2] = rgba[2];
    pixel[3] = rgba[3];
}

fn write_rgb_pixel(color: Color, pixel: &mut [u8]) {
    let rgb = color.to_rgb();
    pixel[0] = rgb[0];
    pixel[1] = rgb[1];
    pixel[2] = rgb[2];
}

fn write_dummy_pixel(_: Color, _: &mut [u8]) {}
