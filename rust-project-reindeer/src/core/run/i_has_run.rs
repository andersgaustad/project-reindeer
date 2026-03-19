use godot::prelude::*;

use crate::core::{audio::sfx_manager::SFXManager, options::options::Options, run::run::Run};


pub trait IHasRun {
    fn get_run(&self) -> Option<Gd<Run>>;


    fn get_options(&self) -> Option<Gd<Options>> {
        self
            .get_run()?
            .bind()
            .get_options()
    }


    fn get_sfx_mananger(&self) -> Option<Gd<SFXManager>> {
        self
            .get_run()?
            .bind()
            .get_sfx_mananger()
    }
}
