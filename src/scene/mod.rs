use crate::ray_tracer::interface::object_base::Object;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
}
