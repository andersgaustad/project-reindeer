use godot::{classes::{Button, CheckButton, Control, DisplayServer, HSlider, IControl, InputEvent, Label, OptionButton, display_server::WindowMode, object::ConnectFlags}, prelude::*};
use strum::IntoEnumIterator;

use crate::{core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, options::option_change::OptionChange, run::{i_has_run::IHasRun, run::Run}, ui::{i_state::IState, options_menu::options_menu_request::OptionsMenuRequest}, utility::node_utility}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct OptionsMenu {
    #[var]
    #[init(node = "%ToggleLowPerformanceModeButton")]
    low_performance_toggle_button : OnReady<Gd<CheckButton>>,

    #[var]
    #[init(node = "%WindowModeOptionButton")]
    window_mode_option_button : OnReady<Gd<OptionButton>>,

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

    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,


    run : Option<Gd<Run>>,


    base : Base<Control>
}


#[godot_api]
impl IControl for OptionsMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();
        
        // Signals

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
        
        self
            .window_mode_option_button
            .signals()
            .item_selected()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_window_mode_option_item_selected
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

        // Focus
        let controls = self.get_focusable_controls_in_order();
        let n_controls = controls.len();

        for i in 0..n_controls {
            let Some(mut control) = controls.get(i).cloned() else {
                continue;
            };

            let north_neighbor_opt = (|| {
                let north_i = i.checked_sub(1)?;
                let north_neighbor_opt = controls.get(north_i).cloned();
                north_neighbor_opt
            })();

            let south_neighbor_opt = controls.get(i + 1).cloned();

            if let Some(north_neighbor) = north_neighbor_opt {
                control.set_focus_neighbor(Side::TOP, &north_neighbor.get_path());
            }

            if let Some(south_neighbor) = south_neighbor_opt {
                control.set_focus_neighbor(Side::BOTTOM, &south_neighbor.get_path());
            }
        }

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        if let Some(run) = self.run.clone() {
            if let Some(options) = run.bind().get_options() {
                options
                    .signals()
                    .option_changed()
                    .builder()
                    .flags(ConnectFlags::DEFERRED)
                    .connect_other_mut(
                        self,
                        Self::on_options_changed
                    );
            }
        }

        // Refresh
        self.refresh();        
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if event.is_action_pressed(UI_CANCEL) {
            self
                .back_button
                .signals()
                .pressed()
                .emit();
        }
    }
}


impl IHasRun for OptionsMenu {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for OptionsMenu {
    fn enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        self.low_performance_toggle_button.grab_focus();

        self.update_ui();
    }


    fn exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);
    }
}


#[godot_api]
impl OptionsMenu {
    #[signal]
    pub fn request(request : OptionsMenuRequest);



    #[func]
    fn on_low_performance_mode_toggled_locally(&mut self, toggled : bool) {
        let options_opt = self.get_options();
        let Some(mut options) = options_opt else {
            return;
        };

        options.bind_mut().set_low_performance_mode(toggled);
    }


    #[func]
    fn on_music_volume_changed_locally(&mut self, value : f64) {
        let Some(mut options) = self.get_options() else {
            return;
        };

        options.bind_mut().set_music_volume(value as f32);        
    }


    #[func]
    fn on_sfx_volume_changed_locally(&mut self, value : f64) {
        let Some(mut options) = self.get_options() else {
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
        let Some(options) = self.get_options() else {
            return;
        };

        let is_toggled = options.bind().get_low_performance_mode();
        self.low_performance_toggle_button.set_pressed(is_toggled);
    }


    #[func]
    fn on_window_mode_option_item_selected(&mut self, id : i64) {
        let Ok(as_usize) = usize::try_from(id) else {
            return;
        };

        let window_modes = window_modes_at_ids();
        let window_mode_opt = window_modes.get(as_usize).cloned();
        let Some(window_mode) = window_mode_opt else {
            return;
        };

        let mut display_server = DisplayServer::singleton();
        display_server.window_set_mode(window_mode);

        self.update_window_mode_button_text();
    }


    #[func]
    fn on_volume_changed(&mut self) {
        let Some(options) = self.get_options() else {
            return;
        };

        let bound_options = options.bind();
        let sfx_factor = bound_options.get_sfx_volume();
        let mut values_and_sliders_and_labels = [
            (bound_options.get_music_volume(), self.music_volume_slider.clone(), self.music_volume_percentage_label.clone()),
            (sfx_factor, self.sfx_volume_slider.clone(), self.sfx_volume_percentage_label.clone()),
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
        self.make_click_sound();
        self
            .signals()
            .request()
            .emit(OptionsMenuRequest::Exit);
    }


    fn refresh(&mut self) {
        for possible_change in OptionChange::iter() {
            self.on_options_changed(possible_change);
        }

        self.update_ui();
    }


    fn update_ui(&mut self) {
        let has_options = self.get_options().is_some();

        let tooltip = if has_options {
            ""
        } else {
            "Could not find Options!"
        };

        let low_performance_toggle_button = &mut self.low_performance_toggle_button;
        low_performance_toggle_button.set_disabled(!has_options);
        low_performance_toggle_button.set_tooltip_text(tooltip);

        self.update_window_mode_button_text();
    }


    fn update_window_mode_button_text(&mut self) {
        let display_server = DisplayServer::singleton();
        let window_mode = display_server.window_get_mode();
        let window_mode_as_str = window_mode.as_str();

        let id_array = window_modes_at_ids();
        let id = id_array.into_iter().position(|window_mode| {
            window_mode.as_str() == window_mode_as_str
        
        }).map(|u| u as i32).unwrap_or(-1);

        self.window_mode_option_button.select(id);
    }


    fn make_click_sound(&mut self) {
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);
    }


    fn get_focusable_controls_in_order(&self) -> [Gd<Control>; 3] {
        let controls = [
            self.low_performance_toggle_button.clone().upcast(),
            self.music_volume_slider.clone().upcast(),
            self.sfx_volume_slider.clone().upcast(),
        ];

        controls
    }
}


// Utility

fn as_percentage_string(number : f32) -> String {
    let percentage = (number * 100f32) as i32;
    let string = format!("{}%", percentage);

    string
}


const fn window_modes_at_ids() -> [WindowMode; 2] {
    [
        WindowMode::WINDOWED,
        WindowMode::FULLSCREEN
    ]
}
