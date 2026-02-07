use godot::prelude::*;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::core::common::coordinate::Coordinate;

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Export, PartialEq, Eq, GodotConvert, Hash, Var)]
#[godot(via = GString)]
pub enum Direction {
    North,
    East,
    South,
    West
}


impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}


impl Direction {
    pub fn rotate_clockwise(&self) -> Self {
        self.rotate_clockwise_by_steps(1)
    }


    pub fn rotate_clockwise_by_steps(&self, steps : usize) -> Self {
        let mut cycle = Direction::iter().cycle();

        let my_index = *self as usize;
        let new_index = my_index + steps;

        // Unwrapping is safe as this is a cycle
        let direction = cycle.nth(new_index).unwrap();

        direction
    }


    pub fn clockwise_rotations_to(&self, other : &Self) -> usize {
        let my_index = *self as usize;
        let other_index = *other as usize;

        (Direction::COUNT + other_index - my_index) % Direction::COUNT
    }


    pub fn counter_clockwise_rotations_to(&self, other : &Self) -> usize {
        Direction::COUNT - self.clockwise_rotations_to(other)
    }


    pub fn to_vector(&self) -> Coordinate {
        Coordinate::from(self)
    }
}
