use godot::{classes::Button, prelude::*};


#[derive(PartialEq, Eq)]
pub(super) enum RebindControlRowState {
    Default,
    ListeningForInput((Gd<Button>, i32)),
    Overshadowed,
}


impl Default for RebindControlRowState {
    fn default() -> Self {
        Self::Default
    }
}
