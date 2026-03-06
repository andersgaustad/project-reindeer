use godot::prelude::*;

use crate::core::options::option_change::OptionChange;


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
    #[signal]
    pub fn option_changed(change : OptionChange);

    #[func]
    pub fn set_low_performance_mode(&mut self, new_mode : bool) {
        let old_mode = self.low_performance_mode;

        // Set
        self.low_performance_mode = new_mode;

        if new_mode != old_mode {
            self.base_mut().emit_changed();
            self.signals().option_changed().emit(OptionChange::LowPerformanceMode);
        }
    }
}
