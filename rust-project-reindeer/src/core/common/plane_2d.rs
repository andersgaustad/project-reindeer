use godot::prelude::*;


pub struct Plane2D {
    normal : Vector2,
    scalar : f32,
}


impl Plane2D {
    pub fn try_from_points(a : Vector2, b : Vector2) -> Option<Self> {
        let delta = a.try_direction_to(b)?;
        let normal = Vector2::new(delta.y, -delta.x);
        let scalar = normal.dot(a);

        let result = Self {
            normal,
            scalar,
        };

        Some(result)
    }


    pub fn distance_to(&self, point : Vector2) -> f32 {
        self.normal.dot(point) - self.scalar
    }
}
