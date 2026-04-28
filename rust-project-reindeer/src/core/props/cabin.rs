use godot::{classes::{CollisionShape3D, GpuParticles3D, StaticBody3D}, prelude::*};

use crate::core::props::flickering_omni_light_3d::FlickeringOmniLight3D;


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct Cabin {
    #[var]
    #[init(node = "%CollisionShape3D")]
    collision : OnReady<Gd<CollisionShape3D>>,

    #[var]
    #[init(node = "%FireLight")]
    fire_light : OnReady<Gd<FlickeringOmniLight3D>>,

    #[var]
    #[init(node = "%SmokeEmitter")]
    smoke_emitter : OnReady<Gd<GpuParticles3D>>,

    base : Base<StaticBody3D>
}


#[godot_api]
impl Cabin {
    pub fn toggle_effects(&mut self, active : bool) {
        let flickering_fireplace_light = &mut self.fire_light;
        flickering_fireplace_light.set_visible(active);
        flickering_fireplace_light.bind_mut().set_flickering_enabled(active);

        self.smoke_emitter.set_visible(active);
        
    }
}
