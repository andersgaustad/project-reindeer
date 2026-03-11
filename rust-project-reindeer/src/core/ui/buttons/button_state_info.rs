use godot::prelude::*;


pub struct ButtonStateInfo {
    pub clickable : bool,

    pub tooltip : GString,
}

impl Default for ButtonStateInfo {
    fn default() -> Self {
        Self {
            clickable : true,
            tooltip : Default::default()
        }
    }
}
