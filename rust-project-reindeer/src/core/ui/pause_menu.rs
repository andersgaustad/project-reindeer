use godot::{classes::{Button, Control, IControl, InputEvent}, prelude::*};

use crate::{core::ui::pause_menu_button_types::PauseMenuButtonType, input_map::CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PauseMenu {
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
    #[init(node = "%MainMenuButton")]
    main_menu_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,

    base : Base<Control>,
}


#[godot_api]
impl IControl for PauseMenu {
    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if !self.base().is_visible() {
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


#[godot_api]
impl PauseMenu {
    #[signal]
    pub fn button_pressed(button_type : PauseMenuButtonType);


    pub fn get_button_from_type(&self, ty : PauseMenuButtonType) -> Gd<Button> {
        match ty {
            PauseMenuButtonType::Start => self.start_button.clone(),
            PauseMenuButtonType::Resume => self.resume_button.clone(),
            PauseMenuButtonType::Options => self.options_button.clone(),
            PauseMenuButtonType::MainMenu => self.main_menu_button.clone(),
            PauseMenuButtonType::Exit => self.exit_button.clone(),
        }
    }
}
