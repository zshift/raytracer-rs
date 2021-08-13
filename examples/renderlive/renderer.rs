use std::f32::consts::E;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

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
            Some(self.aspected_rect(width, height)),
        )?;
        canvas.present();

        Ok(())
    }

    fn scaled_rect(&self) -> Rect {
        let x = (self.x as f32 * self.scale) as i32;
        let y = (self.y as f32 * self.scale) as i32;
        let width = (self.width as f32 * self.scale) as u32;
        let height = (self.height as f32 * self.scale) as u32;

        Rect::new(x, y, width, height)
    }

    fn aspected_rect(&self, width: u32, height: u32) -> Rect {
        let target_aspect = width as f32 / height as f32;
        let img_aspect = self.width as f32 / self.height as f32;

        let x;
        let y;
        let new_width;
        let new_height;

        if img_aspect > target_aspect {
            new_width = width;
            new_height = (new_width as f32 / img_aspect) as u32;
            x = 0;
            y = ((new_height as f32 - height as f32).abs() / 2.) as i32;
        } else {
            new_height = height;
            new_width = (img_aspect * new_height as f32) as u32;
            x = ((new_width as f32 - width as f32).abs() / 2.) as i32;
            y = 0;
        }

        Rect::new(
            x.max(0),
            y.max(0),
            new_width.min(width),
            new_height.min(height),
        )
    }
}
