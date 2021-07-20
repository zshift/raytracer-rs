use crate::{
    element::{Element, Intersection},
    light::Light,
    rendering::{Intersectable, Ray},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
