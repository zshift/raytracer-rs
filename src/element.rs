use crate::{
    color::Color,
    material::Material,
    point::Point,
    rendering::{Intersectable, TextureCoords},
    vector::Vector3,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.coloration.color(coords),
            Element::Plane(ref p) => p.material.coloration.color(coords),
        }
    }

    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }

    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plane {
    pub origin: Point,
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub normal: Vector3,
    pub material: Material,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        Intersection { distance, element }
    }
}
