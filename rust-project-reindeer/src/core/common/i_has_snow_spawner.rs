use godot::{classes::GpuParticles3D, prelude::*};


pub trait IHasSnowSpawner {
    fn get_snow_spawner(&self) -> Gd<GpuParticles3D>;
}
