use godot::{classes::GpuParticles3D, prelude::*};
use strum::IntoEnumIterator;

use crate::core::{blankets::i_change_snow_amout::IChangeSnowAmount, common::i_has_snow_spawner::IHasSnowSpawner, options::option_change::OptionChange, props::cabin::Cabin, run::{i_has_run::IHasRun, run::Run}, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct BackgroundLevel {
    #[var]
    #[init(node = "%Cabin")]
    cabin : OnReady<Gd<Cabin>>,

    #[init(node = "%SnowGPUParticles3D")]
    snow_particle_spawner : OnReady<Gd<GpuParticles3D>>,


    run : Option<Gd<Run>>,


    base : Base<Node3D>,
}


#[godot_api]
impl INode3D for BackgroundLevel {
    fn ready(&mut self) {
        let gd = self.to_gd();
        
        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        let options_opt = self.get_options();
        if let Some(options) = options_opt {
            options
                .signals()
                .option_changed()
                .connect_other(
                    self,
                    Self::on_options_changed
                );
        }

        self.refresh();
    }
}


#[godot_dyn]
impl IHasRun for BackgroundLevel {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


impl IHasSnowSpawner for BackgroundLevel {
    fn get_snow_spawner(&self) -> Gd<GpuParticles3D> {
        self.snow_particle_spawner.clone()
    }
}


#[godot_api]
impl BackgroundLevel {
    #[func]
    fn on_options_changed(&mut self, change : OptionChange) {
        let Some(options) = self.get_options() else {
            return;
        };

        match change {
            OptionChange::LowPerformanceMode => {
                let low_performance_mode = options.bind().get_low_performance_mode();
                self.cabin.bind_mut().toggle_effects(!low_performance_mode);
                self.refresh_snow_amount();
            },
            OptionChange::VolumeChange => {
                // Do nothing
            },
            OptionChange::EffectChange => {
                self.refresh_snow_amount();
            },
        }
    }


    fn refresh(&mut self) {
        for possible_change in OptionChange::iter() {
            self.on_options_changed(possible_change);
        }
    }
}
