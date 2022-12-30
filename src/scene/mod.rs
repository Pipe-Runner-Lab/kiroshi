use crate::ray_tracer::object_base::ObjectBase;

pub struct Scene {
    pub objects: Vec<Box<dyn ObjectBase>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn ObjectBase>) {
        self.objects.push(obj);
    }
}
