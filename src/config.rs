use crate::vec3::Real;

pub struct Config {
    pub width: i32,
    pub height: i32,
    pub samples: i32,
    pub max_depth: i32,
}

impl Config {
    pub fn new(width: i32, height: i32, samples: i32, max_depth: i32) -> Self {
        Self {
            width,
            height,
            samples,
            max_depth,
        }
    }

    pub fn aspect_ratio(&self) -> Real {
        self.width as Real / self.height as Real
    }
}
