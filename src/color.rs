use std::ops::{Add, Mul};

use image::{Pixel, Rgb, Rgba};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Color {
    pub fn clamp(&self) -> Self {
        Self {
            red: self.red.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            0 // ignored
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Self {
        Self {
            red: gamma_decode((rgba[0] as f32) / 255.0),
            green: gamma_decode((rgba[1] as f32) / 255.0),
            blue: gamma_decode((rgba[2] as f32) / 255.0),
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}
