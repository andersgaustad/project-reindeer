use godot::{classes::{GpuParticles3D, OmniLight3D, StaticBody3D}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct Cabin {
    #[var]
    #[init(node = "%FireLight")]
    fire_light : OnReady<Gd<OmniLight3D>>,

    #[var]
    #[init(node = "%SmokeEmitter")]
    smoke_emitter : OnReady<Gd<GpuParticles3D>>,

    base : Base<StaticBody3D>
}


impl Cabin {
    pub fn toggle_effects(&mut self, active : bool) {
        self.fire_light.set_visible(active);    
        self.smoke_emitter.set_visible(active);
    }
}
