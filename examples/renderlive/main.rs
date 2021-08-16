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
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    video::{Window, WindowContext},
};
use std::{fs::File, sync::Arc, time::Duration, usize};
use tokio::sync::{
    oneshot::{self, error::TryRecvError},
    RwLock,
};

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[tokio::main]
async fn main() -> Result<(), String> {
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

    run(scene).await
}
struct Done {}
async fn run(scene: Scene) -> Result<(), String> {
    // Setup a window
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

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Creating canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
    let texture_creator = canvas.texture_creator();

    show_rendering_message(&texture_creator, &mut canvas)?;

    let num_bytes = scene.width * scene.height * ColorType::Rgb8.bytes_per_pixel() as u32;
    let img = vec![0u8; num_bytes as usize];
    let img = Arc::new(RwLock::new(img));
    let img_clone = img.clone();

    let (tx, mut rx) = oneshot::channel();
    tokio::spawn(async move { raytrace(img_clone, scene, tx).await });

    'renderloop: loop {
        match rx.try_recv() {
            Ok(_) => break 'renderloop,
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Closed) => break 'renderloop,
        }
        for event in event_pump.wait_timeout_iter(100) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Exiting before rendering has completed.");
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    let img = img.read().await;
    let texture = texture_creator.load_texture_bytes(&img[..])?;

    let mut drawer = Renderer::new(0, 0, width, height, mouse_state.x(), mouse_state.y());
    drawer.draw(&mut canvas, &texture)?;

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
                    if drawer.is_panning() {
                        drawer.draw(&mut canvas, &texture)?;
                    }
                }
                Event::MouseWheel { direction, y, .. } => {
                    let y = if y > 0 { 1 } else { -1 };
                    let y = y * match direction {
                        MouseWheelDirection::Normal => -1,
                        MouseWheelDirection::Flipped => 1,
                        _ => -1,
                    };
                    drawer.zoom(y as f32);
                    drawer.draw(&mut canvas, &texture)?;
                }

                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Middle,
                    ..
                } => {
                    drawer.start_panning();
                }

                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Middle,
                    ..
                } => {
                    drawer.stop_panning();
                }

                Event::Window {
                    win_event: WindowEvent::Resized(..),
                    ..
                }
                | Event::Window {
                    win_event: WindowEvent::Moved(..),
                    ..
                } => {
                    drawer.draw(&mut canvas, &texture)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn show_rendering_message(
    texture_creator: &TextureCreator<WindowContext>,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    println!("Starting to show message");

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // TODO: replace with a bundled font
    let font = ttf_context.load_font("C:\\Windows\\Fonts\\HARLOWSI.TTF", 128)?;
    let message_surface = font
        .render("Rendering...")
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&message_surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.copy(&texture, None, target)?;
    canvas.present();

    Ok(())
}

async fn raytrace(img: Arc<RwLock<Vec<u8>>>, scene: Scene, done: oneshot::Sender<Done>) {
    let width = scene.width;
    let height = scene.height;
    let color_type = ColorType::Rgb8;
    let bytes_per_pixel = color_type.bytes_per_pixel();
    let num_bytes = width * height * bytes_per_pixel as u32;
    let mut buf = vec![0; num_bytes as usize];
    let mut buf = &mut buf[..];

    raytracer::render(scene, &mut buf, bytes_per_pixel);
    let mut img = img.write().await;
    encode_to_png(buf, &mut img[..], width, height, color_type);

    let _ = done.send(Done {});
}

fn encode_to_png(src: &[u8], dst: &mut [u8], width: u32, height: u32, color_type: ColorType) -> () {
    let png_encoder = PngEncoder::new(dst);
    png_encoder
        .encode(src, width, height, color_type)
        .expect("Convert RGBA to PNG");
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}
