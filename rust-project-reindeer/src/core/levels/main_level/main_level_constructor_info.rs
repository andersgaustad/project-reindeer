use godot::prelude::*;

use crate::core::maze::maze::Maze;


#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct MainLevelConstructorInfo {
    #[var]
    pub maze : Gd<Maze>,

    #[var]
    pub seed : GString,


    base : Base<RefCounted>
}


impl MainLevelConstructorInfo {
    pub fn new(
        maze : Gd<Maze>,
        seed : GString,

    ) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                maze,
                seed,
                base,
            }
        })
    }
}
