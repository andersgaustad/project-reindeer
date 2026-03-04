use godot::prelude::*;


#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Options {
    #[export]
    #[var(get, set = set_low_performance_mode)]
    #[init(val = false)]
    low_performance_mode : bool,

    base : Base<Resource>
}


#[godot_api]
impl Options {
    #[func]
    pub fn set_low_performance_mode(&mut self, new_mode : bool) {
        let old_mode = self.low_performance_mode;

        // Set
        godot_print!(":?- LPC set to {}", new_mode);
        self.low_performance_mode = new_mode;

        if new_mode != old_mode {
            self.base_mut().emit_changed();
        }
    }
}
