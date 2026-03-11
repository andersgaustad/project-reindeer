use godot::prelude::*;

use crate::core::{common::{communicator::Communicator, direction::Direction}, maze::{maze_tile_state::MazeTileState, path_info::PathInfo}};


#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct MazeFindPathsCommunicator {
    #[var(get, set = set_paused)]
    #[init(val = false)]
    paused : bool,

    base : Base<RefCounted>,
}


#[godot_api]
impl MazeFindPathsCommunicator {
    // Signals
    #[signal]
    pub fn start();

    #[signal]
    pub fn unpaused();

    #[signal]
    pub fn update_idx(idx : i32, state : MazeTileState, direction : Direction, acknowledger : Gd<Communicator>);

    #[signal]
    pub fn commit_finished(path_info_opt : Option<Gd<PathInfo>>);

    
    // Methods

    #[func]
    pub fn set_paused(&mut self, value : bool) {
        let previous = self.paused;

        // Set
        self.paused = value;

        let paused_to_unpaused = previous && !value;
        if paused_to_unpaused {
            self
                .signals()
                .unpaused()
                .emit();
        }
    }

    #[func]
    pub fn start(&mut self) {
        self.signals().start().emit();
    }
}
