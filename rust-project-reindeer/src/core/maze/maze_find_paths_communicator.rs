use godot::prelude::*;

use crate::core::{common::{acknowledger::Communicator, direction::Direction}, maze::{maze_tile_state::MazeTileState, path_info::PathInfo}};


#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct MazeFindPathsCommunicator {
    base : Base<RefCounted>,
}


#[godot_api]
impl MazeFindPathsCommunicator {
    // Signals
    #[signal]
    pub fn start();

    #[signal]
    pub fn update_idx(idx : i32, state : MazeTileState, direction : Direction, acknowledger : Gd<Communicator>);

    #[signal]
    pub fn commit_found_path(path_info : Gd<PathInfo>);

    
    // Methods

    #[func]
    pub fn start(&mut self) {
        self.signals().start().emit();
    }
}
