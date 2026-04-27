use godot::{classes::ParticleProcessMaterial, prelude::*};

use crate::core::{common::i_has_snow_spawner::IHasSnowSpawner, run::i_has_run::IHasRun};


pub trait IChangeSnowAmount {
    fn refresh_snow_amount(&mut self);
}


impl<T> IChangeSnowAmount for T
where T : GodotClass + IHasRun + IHasSnowSpawner {
    fn refresh_snow_amount(&mut self) {
        let Some(options) = self.get_options() else {
            return;
        };

        let mut snow_particle_spawner = self.get_snow_spawner();

        let Some(raw_material) = snow_particle_spawner.get_process_material() else {
            return;
        };
        let Ok(particle_process_material) = raw_material.try_cast::<ParticleProcessMaterial>() else {
            return;
        };

        let snow_per_square = options.bind().get_snow_particles_per_square_unit();
        let zero_approx = snow_per_square.is_zero_approx();

        snow_particle_spawner.set_emitting(!zero_approx);
        snow_particle_spawner.set_visible(!zero_approx);

        if zero_approx {
            return;
        }

        // If not zero:

        let particle_spawn_box = particle_process_material.get_emission_box_extents();
        let units = particle_spawn_box.x * particle_spawn_box.z * 4.0;

        let particles = (units * snow_per_square).round() as i32;

        snow_particle_spawner.set_amount(particles);
    }
}
