use godot::prelude::*;

use crate::core::common::direction::Direction;


#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Padding {
    #[export]
    pub north_padding : f32,

    #[export]
    pub east_padding : f32,

    #[export]
    pub south_padding : f32,

    #[export]
    pub west_padding : f32,

    base : Base<Resource>,
}


impl Padding {
    pub fn get_padding_in_direction(&self, direction : Direction) -> f32 {
        match direction {
            Direction::North => self.north_padding,
            Direction::East => self.east_padding,
            Direction::South => self.south_padding,
            Direction::West => self.west_padding,
        }
    }
}
