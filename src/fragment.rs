use nalgebra_glm::Vec2;
use crate::color::Color;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub intensity: f32,  // For lighting calculations
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            intensity: 1.0,  // Default full intensity
        }
    }
    
    pub fn new_with_intensity(x: f32, y: f32, color: Color, depth: f32, intensity: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            intensity,
        }
    }
}
