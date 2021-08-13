use std::f32::consts::E;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const ZOOM_INTENSITY: f32 = 0.1;

pub struct Renderer {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scale: f32,
    zoom: f32,
    mouse_x: i32,
    mouse_y: i32,
}

impl Renderer {
    pub fn new(x: i32, y: i32, width: u32, height: u32, mouse_x: i32, mouse_y: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            mouse_x,
            mouse_y,
            scale: 1.,
            zoom: 1.,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_area(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn zoom(&mut self, direction: f32) {
        self.zoom = E.powf(direction * ZOOM_INTENSITY);
    }

    pub fn set_mouse(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        canvas.copy(
            texture,
            Some(self.scaled_rect()),
            Some(Rect::new(0, 0, self.width, self.height)),
        )?;
        canvas.present();

        Ok(())
    }

    fn scaled_rect(&mut self) -> Rect {
        // TODO: This algorithm sucks. Need to figure it out. 
        let x = self.x
            - (self.mouse_x as f32 / (self.scale * self.zoom) - self.mouse_x as f32 / self.scale)
                as i32;
        let y = self.y
            - (self.y as f32 / (self.scale * self.zoom) - self.mouse_y as f32 / self.scale) as i32;
        let width = (self.width as f32 / self.scale) as u32;
        let height = (self.height as f32 / self.scale) as u32;

        self.scale = (self.scale * self.zoom).clamp(0.2, 4.);
        Rect::new(x, y, width, height)
    }
}
