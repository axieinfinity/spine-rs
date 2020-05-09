#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub in_position: [f32; 2],
    pub in_texture_coords: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct Bounds {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

impl Bounds {
    pub fn dummy() -> Self {
        Self {
            x_min: f32::MAX,
            y_min: f32::MAX,
            x_max: f32::MIN,
            y_max: f32::MIN,
        }
    }

    pub fn cover(&mut self, x: f32, y: f32) {
        *self = Bounds {
            x_min: self.x_min.min(x),
            y_min: self.y_min.min(y),
            x_max: self.x_max.max(x),
            y_max: self.y_max.max(y),
        };
    }
}
