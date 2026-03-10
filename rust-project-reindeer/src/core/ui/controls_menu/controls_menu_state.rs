use godot::{classes::Button, prelude::*};


#[derive(Debug)]
pub enum ControlsMenuState {
    Default,
    WaitingForInput((GString, Gd<Button>)),

}


impl Default for ControlsMenuState {
    fn default() -> Self {
        Self::Default
    }
}
