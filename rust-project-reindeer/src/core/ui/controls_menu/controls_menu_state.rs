use godot::prelude::*;

use crate::core::ui::controls_menu::rebind_control_row::RebindControlRow;


#[derive(Debug)]
pub enum ControlsMenuState {
    Default,
    // RebindControlRow and button ID
    WaitingForInput((Gd<RebindControlRow>, usize)),

}


impl Default for ControlsMenuState {
    fn default() -> Self {
        Self::Default
    }
}
