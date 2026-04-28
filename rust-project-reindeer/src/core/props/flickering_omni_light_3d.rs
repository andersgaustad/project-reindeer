use godot::{classes::{FastNoiseLite, IOmniLight3D, OmniLight3D, Time, light_3d::Param}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=OmniLight3D)]
pub struct FlickeringOmniLight3D {
    #[export]
    #[var]
    noise_texture : OnEditor<Gd<FastNoiseLite>>,

    #[export]
    #[var(set = set_flickering_enabled, get)]
    #[init(val = true)]
    flickering_enabled : bool,

    #[export]
    #[var]
    #[init(val = 0.8)]
    floor_energy : f32,


    default_energy : f32,


    base : Base<OmniLight3D>,
}


#[godot_api]
impl IOmniLight3D for FlickeringOmniLight3D {
    fn ready(&mut self) {
        let energy = self.base().get_param(Param::ENERGY);
        self.default_energy = energy;
    }


    fn process(&mut self, _delta: f64) {
        let time = Time::singleton();
        let seconds = time.get_ticks_msec() as f32 / 1000.0;

        // This is often - but not always - in the range [-1, 1]:
        // https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#class-fastnoiselite
        let raw_noise_value = self.noise_texture.get_noise_1d(seconds);

        // 'Shifted' should be in approximate range of [0, 2].
        let shifted = raw_noise_value + 1.0;

        // Sharpened by cubing: makes energy spikes stronger.
        let sharpened = shifted.powf(3.0);

        // Value has a minimum value to make sure the fire doesn't extinguish.
        let value = (sharpened * self.default_energy).max(self.floor_energy);
        self.set_energy(value);
    }
}


#[godot_api]
impl FlickeringOmniLight3D {
    #[func]
    pub fn set_flickering_enabled(&mut self, enabled : bool) {
        // Set
        self.flickering_enabled = enabled;

        self.base_mut().set_process(enabled);

        if !enabled {
            self.set_energy(self.default_energy);
        }
    }


    pub fn set_energy(&mut self, energy : f32) {
        self.base_mut().set_param(Param::ENERGY, energy);
    }
}
