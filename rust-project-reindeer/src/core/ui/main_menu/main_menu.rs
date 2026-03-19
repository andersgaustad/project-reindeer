use godot::{classes::{AudioStreamPlayer, Button, Control, IControl, object::ConnectFlags}, prelude::*};
use strum::IntoEnumIterator;

use crate::core::{options::{option_change::OptionChange, options::Options}, run::Run, ui::{i_sub_menu_state::IState, main_menu::main_menu_state::MainMenuState}, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenu {
    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%OptionsButton")]
    options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ControlsButton")]
    controls_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%AboutButton")]
    about_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%AudioStreamPlayer")]
    audio_stream_player : OnReady<Gd<AudioStreamPlayer>>,
    default_audio_volume : f32,


    #[var(get, set = set_options)]
    options : Option<Gd<Options>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for MainMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();

        // start_button
        self
            .start_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_start_pressed
            );
        
        // options_button
        self
            .options_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_options_pressed
            );
        
        // controls_button
        self
            .controls_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_controls_pressed
            );
        
        // about_button
        self
            .about_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_about_pressed  
            );

        // exit_button
        self
            .exit_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_exit_pressed
            );
        
        self.default_audio_volume = self.audio_stream_player.get_volume_linear();

        let options_opt = (|| {
            let run = node_utility::try_find_parent_of_type::<Run>(gd.upcast())?;
            let options = run.bind().get_options();
            options
        })();
        
        self.set_options(options_opt);
    }
}


#[godot_dyn]
impl IState for MainMenu {
    fn do_enter(&mut self) {
        self.start_button.grab_focus();
    }


    fn do_exit(&mut self) {
        
    }
}


#[godot_api]
impl MainMenu {
    #[signal]
    pub fn request_state(main_menu_state : MainMenuState);


    #[func]
    pub fn set_options(&mut self, options : Option<Gd<Options>>) {
        // Set
        self.options = options;

        if let Some(some_options) = self.options.clone() {
            some_options
                .signals()
                .option_changed()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    Self::on_option_changed
                );
        }

        for potential_change in OptionChange::iter() {
            self.on_option_changed(potential_change);
        }
    }


    fn on_start_pressed(&mut self) {
        self.make_click_sound();
        self.emit_request_for(MainMenuState::LoadMap)
    }


    fn on_options_pressed(&mut self) {
        self.make_click_sound();
        self.emit_request_for(MainMenuState::Options);
    }


    fn on_controls_pressed(&mut self) {
        self.make_click_sound();
        self.emit_request_for(MainMenuState::Controls);
    }


    fn on_about_pressed(&mut self) {
        self.make_click_sound();
        self.emit_request_for(MainMenuState::About);
    }


    fn on_exit_pressed(&mut self) {
        self.make_click_sound();
        self.base().get_tree().unwrap().quit();
    }


    #[func]
    fn on_option_changed(&mut self, change : OptionChange) {
        let Some(options) = self.options.clone() else {
            return;
        };
        let bound_options = options.bind();

        match change {
            OptionChange::LowPerformanceMode => {
                // Do nothing
            },
            OptionChange::VolumeChange => {
                let sfx_volume_factor = bound_options.get_sfx_volume();
                drop(bound_options);

                let volume = sfx_volume_factor * self.default_audio_volume;
                self.audio_stream_player.set_volume_linear(volume);
            },
        }
    }


    fn emit_request_for(&mut self, state : MainMenuState) {
        self
            .signals()
            .request_state()
            .emit(state);
    }


    fn make_click_sound(&mut self) {
        self.audio_stream_player.play();
    }
}
