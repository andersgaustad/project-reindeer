use godot::{classes::{Button, CheckButton, Control, HSlider, IControl, InputEvent, Label, object::ConnectFlags}, prelude::*};
use strum::IntoEnumIterator;

use crate::{core::{options::{option_change::OptionChange, options::Options}, run::Run, ui::{i_sub_menu_state::IState, options_menu::options_menu_request::OptionsMenuRequest}, utility::node_utility}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct OptionsMenu {
    #[export]
    #[var(get, set = set_options)]
    options : Option<Gd<Options>>,
 

    // Non-exported:

    // Options

    #[var]
    #[init(node = "%ToggleLowPerformanceModeButton")]
    low_performance_toggle_button : OnReady<Gd<CheckButton>>,

    #[var]
    #[init(node = "%MusicVolumeSlider")]
    music_volume_slider : OnReady<Gd<HSlider>>,

    #[var]
    #[init(node = "%MusicVolumePercentageLabel")]
    music_volume_percentage_label : OnReady<Gd<Label>>,

    #[var]
    #[init(node = "%SFXVolumeSlider")]
    sfx_volume_slider : OnReady<Gd<HSlider>>,

    #[var]
    #[init(node = "%SFXVolumePercentageLabel")]
    sfx_volume_percentage_label : OnReady<Gd<Label>>,


    // Back

    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,

    base : Base<Control>
}


#[godot_api]
impl IControl for OptionsMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();
        
        // Signals
        
        // Globals
        let run_opt = node_utility::try_find_parent_of_type::<Run>(gd.upcast());
        if let Some(run) = run_opt {
            let options_opt = run.bind().get_options();
            if let Some(options) = options_opt {
                options
                    .signals()
                    .option_changed()
                    .builder()
                    .flags(ConnectFlags::DEFERRED)
                    .connect_other_mut(
                        self,
                        Self::on_options_changed
                    );
                
                self.options = Some(options);
            }
        }

        // low_performance_toggle_button 
        self
            .low_performance_toggle_button
            .signals()
            .toggled()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_low_performance_mode_toggled_locally
            );
        
        // music_volume_slider
        self
            .music_volume_slider
            .signals()
            .value_changed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
            Self::on_music_volume_changed_locally
        );

        // sfx_volume_slider
        self
            .sfx_volume_slider
            .signals()
            .value_changed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
            Self::on_sfx_volume_changed_locally
        );
        
        // back_button
        self
            .back_button
            .signals()
            .pressed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_back_pressed
            );


        // Refresh
        self.refresh();        
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if !self.base().is_visible_in_tree() {
            return;
        }

        if event.is_action_pressed(UI_CANCEL) {
            self
                .back_button
                .signals()
                .pressed()
                .emit();
        }
    }
}


#[godot_dyn]
impl IState for OptionsMenu {
    fn do_enter(&mut self) {
        self.refresh();
    }
}


#[godot_api]
impl OptionsMenu {
    #[signal]
    pub fn request(request : OptionsMenuRequest);


    #[func]
    pub fn set_options(&mut self, options : Option<Gd<Options>>) {
        // Set
        self.options = options;

        if !self.base().is_node_ready() {
            return;
        }
        let options_opt = self.options.clone();

        let good_config = options_opt.is_some();

        let tooltip = if options_opt.is_some() {
            ""
        } else {
            "Could not find Options!"
        };

        let low_performance_toggle_button = &mut self.low_performance_toggle_button;
        low_performance_toggle_button.set_disabled(!good_config);
        low_performance_toggle_button.set_tooltip_text(tooltip);


        // Sync

        for possible_change in OptionChange::iter() {
            self.on_options_changed(possible_change);
        }
    }


    #[func]
    fn on_low_performance_mode_toggled_locally(&mut self, toggled : bool) {
        let Some(mut options) = self.options.clone() else {
            return;
        };

        options.bind_mut().set_low_performance_mode(toggled);
    }


    #[func]
    fn on_music_volume_changed_locally(&mut self, value : f64) {
        let Some(mut options) = self.options.clone() else {
            return;
        };

        options.bind_mut().set_music_volume(value as f32);        
    }


    #[func]
    fn on_sfx_volume_changed_locally(&mut self, value : f64) {
        let Some(mut options) = self.options.clone() else {
            return;
        };

        options.bind_mut().set_sfx_volume(value as f32);
    }


    #[func]
    fn on_options_changed(&mut self, change : OptionChange) {
        match change {
            OptionChange::LowPerformanceMode => self.on_low_performance_mode_changed(),
            OptionChange::VolumeChange => self.on_volume_changed(),
        }
    }


    #[func]
    fn on_low_performance_mode_changed(&mut self) {
        let Some(options) = self.options.clone() else {
            return;
        };

        let is_toggled = options.bind().get_low_performance_mode();
        self.low_performance_toggle_button.set_pressed(is_toggled);
    }


    #[func]
    fn on_volume_changed(&mut self) {
        let Some(options) = self.options.clone() else {
            return;
        };

        let bound_options = options.bind();
        let mut values_and_sliders_and_labels = [
            (bound_options.get_music_volume(), self.music_volume_slider.clone(), self.music_volume_percentage_label.clone()),
            (bound_options.get_sfx_volume(), self.sfx_volume_slider.clone(), self.sfx_volume_percentage_label.clone()),
        ];
        drop(bound_options);

        for (value, slider, label) in values_and_sliders_and_labels.iter_mut() {
            let value = *value;
            let percentage_string = as_percentage_string(value);

            label.set_text(&percentage_string);
            slider.set_value(f64::from(value));
        }
    }


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(OptionsMenuRequest::Exit);
    }


    fn refresh(&mut self) {
        let options = std::mem::take(&mut self.options);
        self.set_options(options);
    }
}


// Utility

fn as_percentage_string(number : f32) -> String {
    let percentage = (number * 100f32) as i32;
    let string = format!("{}%", percentage);

    string
}
