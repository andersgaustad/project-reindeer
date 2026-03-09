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
    pub fn new(
        maze : Gd<Maze>,
        seed : GString,
        tree_density : f32,
        outer_forest_rings : i32,
        cost_per_rotation : u32,

    ) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            let inner = MainLevelConstructorInfo {
                maze,
                seed,
                tree_density,
                outer_forest_rings,
                cost_per_rotation
            };

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
}
