use godot::{classes::{BoxShape3D, CollisionShape3D, GpuParticles3D, OmniLight3D, StaticBody3D}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct Cabin {
    #[var]
    #[init(node = "%CollisionShape3D")]
    collision : OnReady<Gd<CollisionShape3D>>,

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


    pub fn get_bounding_box(&self) -> Aabb {
        let size = (|| {
            let box_shape = self
                .collision
                .get_shape()?
                .try_cast::<BoxShape3D>()
                .ok()?;

            let size = box_shape.get_size();
            Some(size)

        })().unwrap_or_default();

        let left_front_corner_offset = -size / 2.0;

        let position = self.collision.get_position() + left_front_corner_offset + self.base().get_position();

        let aabb = Aabb::new(
            position,
            size
        );

        aabb
    }
}
