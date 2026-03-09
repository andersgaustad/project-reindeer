use godot::{classes::{Button, Control, IControl, InputEvent, object::ConnectFlags}, prelude::*};
use strum::IntoEnumIterator;

use crate::{core::{levels::main_level::pathfinding_state::PathfindingState, ui::{button_state_info::ButtonStateInfo, i_sub_menu_state::ISubMenuState, pause_menu::{pause_menu_button_types::PauseMenuButtonType, pause_menu_face_request::PauseMenuFaceRequest}}}, input_map::CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PauseMenuFace {
    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ResumeButton")]
    resume_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%OptionsButton")]
    options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ControlsButton")]
    controls_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MainMenuButton")]
    main_menu_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for PauseMenuFace {
    fn ready(&mut self) {
        // Signals

        let buttons_and_pressed_callbacks = PauseMenuButtonType::iter()
            .map(|ty| {
                self.get_button_and_pressed_callback_from_type(ty)
            });
        
        for (button, pressed_callback) in buttons_and_pressed_callbacks {
            button
                .signals()
                .pressed()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    pressed_callback
                );
        }
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if !self.base().is_visible_in_tree() {
            return;
        }

        if event.is_action_pressed(CANCEL) {
            self
                .resume_button
                .signals()
                .pressed()
                .emit();
        }
    }
}


#[godot_dyn]
impl ISubMenuState for PauseMenuFace {
    fn enter(&mut self) {
        self.resume_button.grab_focus();        
    }
}


#[godot_api]
impl PauseMenuFace {
    #[signal]
    pub fn request(request : PauseMenuFaceRequest);


    #[func]
    fn on_start_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(PauseMenuFaceRequest::Start);
    }


    #[func]
    fn on_resume_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(PauseMenuFaceRequest::Resume);
    }


    #[func]
    fn on_options_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(PauseMenuFaceRequest::ToOptions);
    }


    #[func]
    fn on_controls_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(PauseMenuFaceRequest::ToControls);
    }

    #[func]
    fn on_main_menu_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(PauseMenuFaceRequest::ToMainMenu);
    }


    #[func]
    fn on_exit_pressed(&mut self) {
        let Some(mut tree) = self.base().get_tree() else {
            return;
        };

        tree.quit();
    }


    #[func]
    pub fn on_level_pathfinding_state_update(
        &mut self,
        _old_state : PathfindingState,
        new_state : PathfindingState
    ) {
        let default_button_state_info = ButtonStateInfo::default();
        self.set_start_button_state_info(default_button_state_info);

        match new_state {
            PathfindingState::NotStarted => {
                // Do nothing, stick with default
            },
            PathfindingState::Countdown => {

                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Countdown initiated!".into()
                    }
                );
            },
            PathfindingState::InProgress => {
                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Pathfinding is running!".into()
                    }
                );

            },
            PathfindingState::Done => {
                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Pathfinding complete - Exit to Main Menu and reset level to run again.".into()
                    }
                );
            },
        }
    }


    pub fn set_start_button_state_info(&mut self, state_info : ButtonStateInfo) {
        let start_button = &mut self.start_button;
        start_button.set_disabled(!state_info.clickable);
        start_button.set_tooltip_text(&state_info.tooltip);
    }


    fn get_button_and_pressed_callback_from_type(&self, ty : PauseMenuButtonType) -> (Gd<Button>, Box<dyn FnMut(&mut Self) -> ()>) {
        match ty {
            PauseMenuButtonType::Start      => (self.start_button.clone(),      Box::new(Self::on_start_pressed)),
            PauseMenuButtonType::Resume     => (self.resume_button.clone(),     Box::new(Self::on_resume_pressed)),
            PauseMenuButtonType::Options    => (self.options_button.clone(),    Box::new(Self::on_options_pressed)),
            PauseMenuButtonType::Controls   => (self.controls_button.clone(),   Box::new(Self::on_controls_pressed)),
            PauseMenuButtonType::MainMenu   => (self.main_menu_button.clone(),  Box::new(Self::on_main_menu_pressed)),
            PauseMenuButtonType::Exit       => (self.exit_button.clone(),       Box::new(Self::on_exit_pressed)),
        }
    }
}
