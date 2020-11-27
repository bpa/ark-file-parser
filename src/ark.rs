use crate::Location;

pub struct Ark {
    pub x_offset: f32,
    pub x_divisor: f32,
    pub y_offset: f32,
    pub y_divisor: f32,
}

impl Ark {
    pub fn new(x_offset: f32, x_divisor: f32, y_offset: f32, y_divisor: f32) -> Self {
        Ark {
            x_offset,
            x_divisor,
            y_offset,
            y_divisor,
        }
    }

    pub fn longitude(&self, location: &Location) -> f32 {
        self.x_offset + location.x / self.x_divisor
    }

    pub fn latitude(&self, location: &Location) -> f32 {
        self.y_offset + location.y / self.y_divisor
    }
}

//pub const THE_ISLAND: Ark = Ark::new(50.0, 8000.0, 50.0, 8000.0);
