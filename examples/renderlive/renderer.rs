use std::f32::consts::E;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

const ZOOM_INTENSITY: f32 = 0.1;

#[derive(Debug)]
pub struct Renderer {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    mouse: (i32, i32),
    scale: f32,
    zoom: f32,
    panning: bool,
}

impl Renderer {
    pub fn new(x: i32, y: i32, width: u32, height: u32, mouse_x: i32, mouse_y: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            mouse: (mouse_x, mouse_y),
            scale: 1.,
            zoom: 1.,
            panning: false,
        }
    }

    pub fn zoom(&mut self, direction: f32) {
        self.zoom = E.powf(direction * ZOOM_INTENSITY);
        self.scale *= self.zoom;
    }

    pub fn set_mouse(&mut self, x: i32, y: i32) {
        self.mouse = (x, y);
    }

    pub fn start_panning(&mut self) {
        self.panning = true;
    }

    pub fn stop_panning(&mut self) {
        self.panning = false;
    }

    pub fn is_panning(&self) -> bool {
        self.panning
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.copy(
            texture,
            Some(self.scaled_rect()),
            Some(self.aspected_rect(width as f32, height as f32)),
        )?;
        canvas.present();

        Ok(())
    }

    fn scaled_rect(&self) -> Rect {
        let x = self.x as f32 * self.scale;
        let y = self.y as f32 * self.scale;
        let width = self.width as f32 * self.scale;
        let height = self.height as f32 * self.scale;

        rect!(x, y, width, height)
    }

    fn aspected_rect(&self, width: f32, height: f32) -> Rect {
        let target_aspect = width / height;
        let img_aspect = self.width as f32 / self.height as f32;

        let x;
        let y;
        let new_width;
        let new_height;

        if img_aspect > target_aspect {
            new_width = width;
            new_height = new_width / img_aspect;
            x = 0.;
            y = (new_height - height).abs() / 2.;
        } else {
            new_height = height;
            new_width = img_aspect * new_height;
            x = (new_width - width).abs() / 2.;
            y = 0.;
        }

        rect!(
            x.max(0.),
            y.max(0.),
            new_width.min(width),
            new_height.min(height)
        )
    }
}
