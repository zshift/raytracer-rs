use clap::{App, Arg};
use image::ColorType;
use raytracer::scene::Scene;
use std::{fs::File, time, usize};

fn main() {
    let app = App::new("raytracer")
        .version("0.1.0")
        .author("Peter Faria <zshift@gmail.com>")
        .about("Simple raytracer implementation (source: https://www.github.com/zshift/raytracer-rs")
        .arg(Arg::with_name("scene")
            .help("Sets the scene file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("image")
            .help("Sets the output image file")
            .required(true)
            .index(2));

    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let width = scene.width;
    let height = scene.height;
    let color_type = ColorType::Rgb8;
    let bytes_per_pixel = color_type.bytes_per_pixel();

    let num_bytes = width as usize * height as usize * bytes_per_pixel as usize;
    let mut buf = vec![0; num_bytes];
    let mut buf = &mut buf[..];

    let start = time::Instant::now();
    println!("Starting rendering at {:?}", start);
    raytracer::render(scene, &mut buf, bytes_per_pixel);
    let dur = time::Instant::now() - start;
    println!("Finished rendering.\nRender time: {:?}\n", dur);

    let start = time::Instant::now();
    println!("Starting file save at {:?}", start);
    if let Some(e) = image::save_buffer(image_path, &buf, width, height, color_type).err() {
        println!("Failed to save image: {}", e);
    } else {
        let dur = time::Instant::now() - start;
        println!("Finished saving.\nSave time: {:?}\n", dur);
    }
}
