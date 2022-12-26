use crate::utils::vec4::Point;

pub struct Ray {
    pub origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }
}