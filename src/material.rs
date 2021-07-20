use std::{fmt, path::PathBuf};

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Deserializer};

use crate::{color::Color, rendering::TextureCoords};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Texture {
    pub path: PathBuf,

    #[serde(skip_serializing, skip_deserializing, default = "dummy_texture")]
    pub texture: DynamicImage,
}
fn dummy_texture() -> DynamicImage {
    DynamicImage::new_rgb8(0, 0)
}
impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Texture({:?})", self.path)
    }
}
fn load_texture<'de, D>(deserializer: D) -> Result<Texture, D::Error>
where
    D: Deserializer<'de>,
{
    let texture = Texture::deserialize(deserializer)?;
    if let Ok(img) = image::open(texture.path.clone()) {
        Ok(Texture {
            path: texture.path,
            texture: img,
        })
    } else {
        Err(::serde::de::Error::custom(format!(
            "Unable to open texture file: {:?}",
            texture.path
        )))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Coloration {
    Color(Color),
    Texture(#[serde(deserialize_with = "load_texture")] Texture),
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Coloration {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(ref c) => c.clone(),
            Coloration::Texture(ref tex) => {
                let tex_x = wrap(coords.x, tex.texture.width());
                let tex_y = wrap(coords.y, tex.texture.height());

                Color::from_rgba(tex.texture.get_pixel(tex_x, tex_y)).clone()
            }
        }
    }
}