use godot::{classes::AudioStreamPlayer, prelude::*};
use strum::{EnumCount, IntoEnumIterator, VariantArray};

use crate::core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, options::option_change::OptionChange, run::{i_has_run::IHasRun, run::Run}, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SFXManager {
    #[var]
    #[init(node = "%Click")]
    click : OnReady<Gd<AudioStreamPlayer>>,
    default_click_volume : f32,

    #[var]
    #[init(node = "%Error")]
    error : OnReady<Gd<AudioStreamPlayer>>,
    default_error_volume : f32,

    #[var]
    #[init(node = "%RebindStart")]
    rebind_start : OnReady<Gd<AudioStreamPlayer>>,
    default_rebind_start_volume : f32,

    #[var]
    #[init(node = "%RebindEnd")]
    rebind_end : OnReady<Gd<AudioStreamPlayer>>,
    default_rebind_end_volume : f32,


    run : Option<Gd<Run>>,


    base : Base<Node>,
}



#[godot_api]
impl INode for SFXManager {
    fn ready(&mut self) {
        let gd = self.to_gd();

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        for entry in SFXEntry::iter() {
            let (sfx, value) = self.get_sfx_and_base_mut(entry);
            *value = sfx.get_volume_linear();
        }

        self.refresh();
    }
}


#[godot_dyn]
impl IHasRun for SFXManager {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl ISFXManager for SFXManager {
    fn play(&mut self, entry : SFXEntry) {
        let (mut sfx, _) = self.get_sfx_and_base_mut(entry);
        sfx.play();
    }
}


#[godot_api]
impl SFXManager {
    #[func]
    fn on_option_change(&mut self, change : OptionChange) {
        let Some(option) = self.get_options() else {
            return;
        };

        match change {
            OptionChange::LowPerformanceMode => {
                // Do nothing
            },
            OptionChange::VolumeChange => {
                let sfx_factor = option.bind().get_sfx_volume();

                let sfxs_and_bases = self.get_all_sfxs_and_bases();
                for (mut sfx, base_volume) in sfxs_and_bases {
                    let new_volume = sfx_factor * base_volume;
                    sfx.set_volume_linear(new_volume);
                }
            },
        }
    }


    fn refresh(&mut self) {
        for potential_change in OptionChange::iter() {
            self.on_option_change(potential_change);
        }
    }


    fn get_sfx_and_base(&self, entry : SFXEntry) -> (Gd<AudioStreamPlayer>, f32) {
        match entry {
            SFXEntry::Click         => (self.click.clone(), self.default_click_volume),
            SFXEntry::Error         => (self.error.clone(), self.default_error_volume),
            SFXEntry::RebindStart   => (self.rebind_start.clone(), self.default_rebind_start_volume),
            SFXEntry::RebindEnd     => (self.rebind_end.clone(), self.default_rebind_end_volume),
        }
    }

    fn get_sfx_and_base_mut(&mut self, entry : SFXEntry) -> (Gd<AudioStreamPlayer>, &mut f32) {
        match entry {
            SFXEntry::Click         => (self.click.clone(), &mut self.default_click_volume),
            SFXEntry::Error         => (self.error.clone(), &mut self.default_error_volume),
            SFXEntry::RebindStart   => (self.rebind_start.clone(), &mut self.default_rebind_start_volume),
            SFXEntry::RebindEnd     => (self.rebind_end.clone(), &mut self.default_rebind_end_volume),
        }
    }


    fn get_all_sfxs_and_bases(&self) -> [(Gd<AudioStreamPlayer>, f32); SFXEntry::COUNT] {
        let entries = self.get_sfx_entries();
        let mapped = entries.map(|entry| self.get_sfx_and_base(entry));

        mapped
    }


    fn get_sfx_entries(&self) -> [SFXEntry; SFXEntry::COUNT] {
        SFXEntry::VARIANTS.try_into().unwrap()
    }
}


// Extended ISFXManager

impl ISFXManager for Option<Gd<SFXManager>> {
    fn play(&mut self, entry : SFXEntry) {
        if let Some(some) = self.clone() {
            some.into_dyn::<dyn ISFXManager>().dyn_bind_mut().play(entry);

        } else {
            godot_warn!("Tried to play entry from None::<SFXManager>.");
        }
    }
}
