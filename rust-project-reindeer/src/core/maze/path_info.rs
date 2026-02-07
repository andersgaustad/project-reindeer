use godot::prelude::*;

use crate::core::common::coordinate::Coordinate;


#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct PathInfo {
    paths : Vec<Vec<Coordinate>>,
    score : usize,

    base : Base<RefCounted>
}


impl PathInfo {
    pub fn new_gd(
        paths : Vec<Vec<Coordinate>>,
        score : usize,

    ) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                paths,
                score,
                base,
            }
        })
    }


    pub fn rust_get_paths(&self) -> &Vec<Vec<Coordinate>> {
        &self.paths
    }


    pub fn rust_get_score(&self) -> usize {
        self.score
    }
}
