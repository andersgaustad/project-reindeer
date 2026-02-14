use godot::prelude::*;

use crate::core::maze::maze::Maze;


pub struct MazeInfo {
    pub maze : Gd<Maze>,
    pub size : Vector2,
}
