use godot::prelude::*;

use crate::core::common::direction::Direction;


#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Reindeer {
    #[export]
    #[var(get, set = set_reindeer_rotation)]
    #[init(val = Direction::South)]
    reindeer_rotation : Direction,

    #[var]
    #[init(node = "%Body")]
    body : OnReady<Gd<Node3D>>,

    base : Base<Node3D>
}


#[godot_api]
impl INode3D for Reindeer {
    fn ready(&mut self) {
        let rotation = self.reindeer_rotation;
        self.set_reindeer_rotation(rotation);
    }
}


#[godot_api]
impl Reindeer {
    #[func]
    pub fn set_reindeer_rotation(&mut self, value : Direction) {
        self.reindeer_rotation = value;

        let rotate_steps = match self.reindeer_rotation {
            Direction::South => 0.0,
            Direction::West => 1.0,
            Direction::North => 2.0,
            Direction::East => 3.0,
        };

        const QUARTER_CIRCLE : f32 = std::f32::consts::FRAC_PI_2;
        
        let radians = QUARTER_CIRCLE * rotate_steps;

        self.body.set_rotation(Vector3::new(0.0, radians, 0.0)); 
    }
}
