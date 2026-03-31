use godot::prelude::*;

use crate::core::maze::maze::Maze;


// GodotMainLevelConstructorInfo

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct GodotMainLevelConstructorInfo {
    pub inner : MainLevelConstructorInfo,

    base : Base<RefCounted>
}


impl GodotMainLevelConstructorInfo {
    pub fn new(inner : MainLevelConstructorInfo,) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner,
                base,
            }
        })
    }
}


// MainLevelConstructorInfo

#[derive(Clone)]
pub struct MainLevelConstructorInfo {
    pub maze : Gd<Maze>,

    pub seed : GString,

    pub tree_density : f32,

    pub outer_forest_rings : i32,

    pub cost_per_rotation : u32,

    pub color_a : Color,

    pub color_b : Color,
}
