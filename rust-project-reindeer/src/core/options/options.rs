use godot::prelude::*;

use crate::core::options::option_change::OptionChange;


#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Options {
    #[export_group(name = "Performance")]

    #[export]
    #[var(get, set = set_low_performance_mode)]
    #[init(val = false)]
    low_performance_mode : bool,


    #[export_group(name = "Volume")]

    #[export(range=(0.0, 2.0))]
    #[var(get, set = set_music_volume)]
    #[init(val = 1.0)]
    music_volume : f32,

    #[export(range=(0.0, 2.0))]
    #[var(get, set = set_sfx_volume)]
    #[init(val = 1.0)]
    sfx_volume : f32,


    base : Base<Resource>
}


#[godot_api]
impl Options {
    #[signal]
    pub fn option_changed(change : OptionChange);


    #[func]
    pub fn set_low_performance_mode(&mut self, new_mode : bool) {
        // Set
        let old_mode = std::mem::replace(&mut self.low_performance_mode, new_mode);

        if new_mode != old_mode {
            self.base_mut().emit_changed();
            self.signals().option_changed().emit(OptionChange::LowPerformanceMode);
        }
    }


    #[func]
    pub fn set_music_volume(&mut self, new_factor : f32) {
        // Set
        let old_factor = std::mem::replace(&mut self.music_volume, new_factor);

        if new_factor != old_factor {
            self.base_mut().emit_changed();
            self.signals().option_changed().emit(OptionChange::VolumeChange);
        }

    }


    #[func]
    pub fn set_sfx_volume(&mut self, new_factor : f32) {
        // Set
        let old_factor = std::mem::replace(&mut self.sfx_volume, new_factor);

        if new_factor != old_factor {
            self.base_mut().emit_changed();
            self.signals().option_changed().emit(OptionChange::VolumeChange);
        }
    }
}
