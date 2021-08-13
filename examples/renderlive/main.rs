mod renderer;

use clap::{App, Arg};
use image::{png::PngEncoder, ColorType};
use raytracer::scene::Scene;
use renderer::Renderer;
use sdl2::{
    event::{Event, WindowEvent},
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    mouse::MouseWheelDirection,
};
use std::{fs::File, usize};

fn main() {
    let app = App::new("raytracer")
        .version("0.1.0")
        .author("Peter Faria <zshift@gmail.com>")
        .about(
            "Simple raytracer implementation (source: https://www.github.com/zshift/raytracer-rs",
        )
        .arg(
            Arg::with_name("scene")
                .help("Sets the scene file to use")
                .required(true)
                .index(1),
        );

    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    run(scene);
}

fn run(scene: Scene) {
    let width = scene.width;
    let height = scene.height;
    let color_type = ColorType::Rgb8;
    let bytes_per_pixel = color_type.bytes_per_pixel();

    let num_bytes = width as usize * height as usize * bytes_per_pixel as usize;
    let mut buf = vec![0; num_bytes];
    let mut buf = &mut buf[..];

    raytracer::render(scene, &mut buf, bytes_per_pixel);

    let mut img = vec![0u8; num_bytes];
    encode_to_png(buf, &mut img[..], width, height, color_type);

    match display_image(&img[..]) {
        Ok(_) => {}
        Err(e) => println!("Failed to display image: {}", e),
    }
}

fn encode_to_png(src: &[u8], dst: &mut [u8], width: u32, height: u32, color_type: ColorType) -> () {
    let png_encoder = PngEncoder::new(dst);
    png_encoder
        .encode(src, width, height, color_type)
        .expect("Convert RGBA to PNG");
}

fn display_image(img: &[u8]) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let width = 800;
    let height = 600;

    let window = video_subsystem
        .window("Raytracing", width, height)
        .vulkan()
        .position_centered()
        .resizable()
        .build()
        .expect("Creating window");

    let (x, y) = window.position();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Creating canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
    let mut drawer = Renderer::new(x, y, width, height, mouse_state.x(), mouse_state.y());

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(img)?;

    'miniloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'miniloop,
                Event::MouseMotion { x, y, .. } => {
                    drawer.set_mouse(x, y);
                }
                Event::MouseWheel { direction, y, .. } => {
                    let y = y as f32
                        * match direction {
                            MouseWheelDirection::Normal => 1.,
                            MouseWheelDirection::Flipped => -1.,
                            _ => 1.,
                        };
                    drawer.zoom(y);
                    drawer.draw(&mut canvas, &texture)?;
                }

                Event::Window {
                    win_event: WindowEvent::Moved(x, y),
                    ..
                } => {
                    drawer.set_position(x, y);
                    drawer.draw(&mut canvas, &texture)?;
                }

                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    drawer.set_area(width as u32, height as u32);
                    drawer.draw(&mut canvas, &texture)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
