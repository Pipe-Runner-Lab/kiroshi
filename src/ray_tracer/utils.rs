use crate::utils::vec4::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }
}

pub fn map_to_range(value: f32, from_min: f32, from_max:f32, to_min: f32, to_max: f32) -> f32 {
    // brings start of series to 0
    let diff = -from_min;
    let mut new_value = value + diff;
    // scales the range between 0 and 1
    let range = from_max - from_min;
    new_value /= range;

    to_min + new_value * (to_max - to_min)
}