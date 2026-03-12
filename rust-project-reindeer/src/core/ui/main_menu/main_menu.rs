use godot::{classes::{Button, Control, IControl}, prelude::*};

use crate::core::ui::{i_sub_menu_state::IState, main_menu::main_menu_state::MainMenuState};


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


    base : Base<Control>,
}


#[godot_api]
impl IControl for MainMenu {
    fn ready(&mut self) { 
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


    fn on_start_pressed(&mut self) {
        self.emit_request_for(MainMenuState::LoadMap)
    }


    fn on_options_pressed(&mut self) {
        self.emit_request_for(MainMenuState::Options);
    }


    fn on_controls_pressed(&mut self) {
        self.emit_request_for(MainMenuState::Controls);
    }


    fn on_about_pressed(&mut self) {
        self.emit_request_for(MainMenuState::About);
    }


    fn on_exit_pressed(&mut self) {
        self.base().get_tree().unwrap().quit();
    }


    fn emit_request_for(&mut self, state : MainMenuState) {
        self
            .signals()
            .request_state()
            .emit(state);
    }
}
