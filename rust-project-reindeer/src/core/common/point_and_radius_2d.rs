use godot::prelude::*;


pub struct PointAndRadius2D {
    pub point : Vector2,

    pub radius : f32,
}


impl PointAndRadius2D {
    pub fn new(
        point : Vector2,
        radius : f32,

    ) -> Self {
        Self {
            point,
            radius
        }
    }
}
