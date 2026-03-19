use godot::{classes::{AudioStreamPlayer, object::ConnectFlags}, prelude::*, register::ConnectHandle};
use strum::{EnumCount, IntoEnumIterator, VariantArray};

use crate::core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, options::{option_change::OptionChange, options::Options}, run::Run, utility::node_utility};


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


    #[var(get, set = set_options)]
    options : Option<Gd<Options>>,
    option_listeners : Vec<ConnectHandle>,


    base : Base<Node>,
}



#[godot_api]
impl INode for SFXManager {
    fn ready(&mut self) {
        let gd = self.to_gd();

        let options_opt = (|| {
            let run = node_utility::try_find_parent_of_type::<Run>(gd.upcast())?;
            let options = run.bind().get_options();
            options
        })();
        self.options = options_opt;

        self.refresh();
    }
}


#[godot_dyn]
impl ISFXManager for SFXManager {
    fn play(&mut self, entry : SFXEntry) {
        let (mut sfx, _) = self.get_sfx_and_base(entry);
        sfx.play();
    }
}


#[godot_api]
impl SFXManager {
    #[func]
    pub fn set_options(&mut self, options : Option<Gd<Options>>) {
        // Set
        self.options = options;

        let old_option_listeners = std::mem::take(&mut self.option_listeners);
        for old_listener in old_option_listeners {
            old_listener.disconnect();
        }
        
        if let Some(options) = self.options.clone() {
            let handle = options
                .signals()
                .option_changed()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    Self::on_option_change
                );
            
            self.option_listeners.push(handle);
        }

        for potential_change in OptionChange::iter() {
            self.on_option_change(potential_change);
        }
    }


    #[func]
    fn on_option_change(&mut self, change : OptionChange) {
        let Some(option) = self.options.clone() else {
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
        let options = std::mem::take(&mut self.options);
        self.set_options(options);
    }


    fn get_sfx_and_base(&self, entry : SFXEntry) -> (Gd<AudioStreamPlayer>, f32) {
        match entry {
            SFXEntry::Click => (self.click.clone(), self.default_click_volume),
            SFXEntry::Error => (self.error.clone(), self.default_error_volume),
        }
    }


    fn get_all_sfxs_and_bases(&self) -> [(Gd<AudioStreamPlayer>, f32); SFXEntry::COUNT] {
        let entries : [SFXEntry; SFXEntry::COUNT] = SFXEntry::VARIANTS.try_into().unwrap();
        let mapped = entries.map(|entry| self.get_sfx_and_base(entry));

        mapped
    }
}


// Extended ISFXManager

impl ISFXManager for Option<Gd<SFXManager>> {
    fn play(&mut self, entry : SFXEntry) {
        if let Some(some) = self.clone() {
            some.into_dyn().dyn_bind_mut().play(entry);

        } else {
            godot_warn!("Tried to play entry from None::<SFXManager>.");
        }
    }
}
